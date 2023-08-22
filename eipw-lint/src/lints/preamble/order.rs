/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Order<S>(pub Vec<S>);

impl<S> Order<S>
where
    S: AsRef<str>,
{
    fn find_preceding(&self, present: &[&str], needle: &str) -> Option<&str> {
        let needle_idx = match self.0.iter().position(|x| x.as_ref() == needle) {
            None | Some(0) => return None,
            Some(i) => i,
        };

        for (idx, name) in self.0.iter().enumerate().rev() {
            let name = name.as_ref();
            if name != needle && present.contains(&name) && idx < needle_idx {
                return Some(name);
            }
        }

        None
    }
}

impl<S> Lint for Order<S>
where
    S: Debug + Display + AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        // Check for unknown headers.
        let unknowns: Vec<_> = ctx
            .preamble()
            .fields()
            .filter(|f| !self.0.iter().any(|e| e == &f.name()))
            .map(|f| Slice {
                line_start: f.line_start(),
                fold: false,
                origin: ctx.origin(),
                source: f.source(),
                annotations: vec![SourceAnnotation {
                    annotation_type: ctx.annotation_type(),
                    label: "unrecognized header",
                    range: (0, f.name().chars().count()),
                }],
            })
            .collect();

        if !unknowns.is_empty() {
            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: ctx.annotation_type(),
                    label: Some("preamble has extra header(s)"),
                }),
                footer: vec![],
                slices: unknowns,
                opt: Default::default(),
            })?;
        }

        let present: Vec<_> = ctx.preamble().fields().map(|f| f.name()).collect();

        // Check that headers are in the correct order.
        let mut max_line = 0;
        for name in self.0.iter() {
            if let Some(field) = ctx.preamble().by_name(name.as_ref()) {
                let cur = max_line;
                max_line = field.line_start();

                if max_line >= cur {
                    continue;
                }

                let label = format!("preamble header `{}` is out of order", field.name());
                let mut footer_label = String::new();
                let mut footer = vec![];

                if let Some(preceding) = self.find_preceding(&present, field.name()) {
                    write!(
                        footer_label,
                        "`{}` should come after `{}`",
                        field.name(),
                        preceding,
                    )
                    .unwrap();

                    footer.push(Annotation {
                        annotation_type: AnnotationType::Help,
                        id: None,
                        label: Some(&footer_label),
                    });
                }

                ctx.report(Snippet {
                    title: Some(Annotation {
                        id: Some(slug),
                        annotation_type: ctx.annotation_type(),
                        label: Some(&label),
                    }),
                    footer,
                    slices: vec![Slice {
                        line_start: field.line_start(),
                        origin: ctx.origin(),
                        source: field.source(),
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
