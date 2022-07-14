/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug)]
pub struct SectionOrder<'n>(pub &'n [&'n str]);

impl<'n> SectionOrder<'n> {
    fn find_preceding(&self, present: &[&str], needle: &str) -> Option<&str> {
        let needle_idx = match self.0.iter().position(|x| *x == needle) {
            None | Some(0) => return None,
            Some(i) => i,
        };

        for (idx, name) in self.0.iter().enumerate().rev() {
            if *name != needle && present.contains(name) && idx < needle_idx {
                return Some(name);
            }
        }

        None
    }
}

impl<'n> Lint for SectionOrder<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        // Collect the headings.
        let headings_bytes = ctx
            .body()
            .descendants()
            // Find all headings of level 2.
            .filter_map(|start| match &*start.data.borrow() {
                Ast {
                    value: NodeValue::Heading(NodeHeading { level: 2, .. }),
                    start_line,
                    ..
                } => Some((*start_line, start)),
                _ => None,
            })
            // Descend into their children.
            .map(|(start_line, heading)| {
                let collected: Vec<_> = heading
                    .descendants()
                    .skip(1)
                    // Filter for text nodes.
                    .filter_map(|child| match &*child.data.borrow() {
                        Ast {
                            value: NodeValue::Text(v),
                            ..
                        } => Some(v.to_vec()),
                        _ => None,
                    })
                    .flatten()
                    .collect();
                (start_line, collected)
            });

        let mut headings = Vec::new();
        for (line_start, bytes) in headings_bytes {
            headings.push((line_start, String::from_utf8(bytes)?));
        }

        // Check for unknown sections.
        let unknowns: Vec<_> = headings
            .iter()
            .filter(|(_, f)| !self.0.contains(&f.as_str()))
            .map(|(line_start, _)| Slice {
                line_start: usize::try_from(*line_start).unwrap(),
                fold: false,
                origin: ctx.origin(),
                source: ctx.line(*line_start),
                annotations: vec![],
            })
            .collect();

        if !unknowns.is_empty() {
            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: AnnotationType::Error,
                    label: Some("body has extra section(s)"),
                }),
                footer: vec![],
                slices: unknowns,
                opt: Default::default(),
            })?;
        }

        // Check that sections are in the correct order.
        let map: HashMap<_, _> = headings.into_iter().map(|(a, b)| (b, a)).collect();
        let present: Vec<_> = map.keys().map(String::as_str).collect();

        let mut max_line = 0;
        for name in self.0.iter() {
            if let Some(line_start) = map.get(*name).copied() {
                let cur = max_line;
                max_line = line_start;

                if max_line >= cur {
                    continue;
                }

                let label = format!("section `{}` is out of order", name);
                let mut footer_label = String::new();
                let mut footer = vec![];

                if let Some(preceding) = self.find_preceding(&present, name) {
                    write!(footer_label, "`{}` should come after `{}`", name, preceding,).unwrap();

                    footer.push(Annotation {
                        annotation_type: AnnotationType::Help,
                        id: None,
                        label: Some(&footer_label),
                    });
                }

                ctx.report(Snippet {
                    title: Some(Annotation {
                        id: Some(slug),
                        annotation_type: AnnotationType::Error,
                        label: Some(&label),
                    }),
                    footer,
                    slices: vec![Slice {
                        line_start: line_start.try_into().unwrap(),
                        origin: ctx.origin(),
                        source: ctx.line(line_start),
                        fold: false,
                        annotations: vec![],
                    }],
                    opt: Default::default(),
                })?;
            }
        }

        Ok(())
    }
}
