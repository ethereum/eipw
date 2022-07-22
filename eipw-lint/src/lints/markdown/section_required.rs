/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct SectionRequired<'n>(pub &'n [&'n str]);

impl<'n> Lint for SectionRequired<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        // Collect the headings.
        let headings: Vec<_> = ctx
            .body()
            .descendants()
            // Find all headings of level 2.
            .filter(|start| {
                matches!(
                    &*start.data.borrow(),
                    Ast {
                        value: NodeValue::Heading(NodeHeading { level: 2, .. }),
                        ..
                    }
                )
            })
            // Descend into their children.
            .map(|heading| {
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
                collected
            })
            .filter_map(|v| String::from_utf8(v).ok())
            .collect();

        // Use a `Vec` here to preserve the order of sections.
        let mut missing = self.0.to_vec();

        // TODO: I'm sure this is horribly inefficient!
        missing.retain(|i| {
            for text in &headings {
                if *i == text {
                    return false;
                }
            }
            true
        });

        if missing.is_empty() {
            return Ok(());
        }

        let label = format!("body is missing section(s): `{}`", missing.join("`, `"));
        ctx.report(Snippet {
            title: Some(Annotation {
                annotation_type: AnnotationType::Error,
                id: Some(slug),
                label: Some(&label),
            }),
            slices: vec![Slice {
                fold: true,
                annotations: vec![],
                origin: ctx.origin(),
                source: ctx.body_source(),
                line_start: ctx.body().data.borrow().start_line.try_into().unwrap(),
            }],
            footer: vec![],
            opt: Default::default(),
        })?;

        Ok(())
    }
}
