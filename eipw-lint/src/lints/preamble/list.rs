/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct List<S>(pub S);

impl<S> Lint for List<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let mut missing_space = Vec::new();
        let mut extra_space = Vec::new();

        let value = field.value().trim();
        if value.is_empty() {
            return Ok(());
        }

        let mut offset = 0;
        for matched in value.split(',') {
            let current = offset;
            offset += matched.chars().count() + 1;

            let name_count = field.name().chars().count();

            let trimmed = matched.trim();
            if trimmed.is_empty() {
                let label = format!("preamble header `{}` cannot have empty items", self.0);
                ctx.report(Snippet {
                    title: Some(Annotation {
                        annotation_type: ctx.annotation_type(),
                        id: Some(slug),
                        label: Some(&label),
                    }),
                    footer: vec![],
                    slices: vec![Slice {
                        fold: false,
                        line_start: field.line_start(),
                        origin: ctx.origin(),
                        source: field.source(),
                        annotations: vec![SourceAnnotation {
                            annotation_type: ctx.annotation_type(),
                            label: "this item is empty",
                            range: (name_count + current + 1, name_count + current + 2),
                        }],
                    }],
                    opt: Default::default(),
                })?;
                continue;
            }

            let rest = match matched.strip_prefix(' ') {
                Some(r) => r,
                None if current == 0 => matched,
                None => {
                    missing_space.push(SourceAnnotation {
                        annotation_type: ctx.annotation_type(),
                        label: "missing space",
                        range: (name_count + current + 1, name_count + current + 2),
                    });
                    continue;
                }
            };

            if rest.trim() == rest {
                continue;
            }

            extra_space.push(SourceAnnotation {
                annotation_type: ctx.annotation_type(),
                label: "extra space",
                range: (
                    name_count + current + 2,
                    name_count + current + 2 + matched.chars().count(),
                ),
            });
        }

        if !missing_space.is_empty() {
            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some("preamble header list items must begin with a space"),
                }),
                footer: vec![],
                slices: vec![Slice {
                    line_start: field.line_start(),
                    fold: false,
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: missing_space,
                }],
                opt: Default::default(),
            })?;
        }

        if !extra_space.is_empty() {
            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some("preamble header list items have extra whitespace"),
                }),
                footer: vec![],
                slices: vec![Slice {
                    line_start: field.line_start(),
                    fold: false,
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: extra_space,
                }],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
