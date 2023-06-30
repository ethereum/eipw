/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Trim;

impl Lint for Trim {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let mut no_space = Vec::new();

        for field in ctx.preamble().fields() {
            let mut value = field.value();
            if value.is_empty() {
                continue;
            }

            if let Some(v) = value.strip_prefix(' ') {
                value = v;
            } else {
                no_space.push(field);
            }

            if value.trim() == value {
                continue;
            }

            let name_count = field.name().chars().count();
            let value_count = field.value().chars().count();

            let label = format!("preamble header `{}` has extra whitespace", field.name());
            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    line_start: field.line_start(),
                    fold: false,
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: vec![SourceAnnotation {
                        annotation_type: ctx.annotation_type(),
                        label: "value has extra whitespace",
                        range: (name_count + 1, value_count + name_count + 1),
                    }],
                }],
                footer: vec![],
                opt: Default::default(),
            })?;
        }

        if !no_space.is_empty() {
            let slices = no_space
                .into_iter()
                .map(|n| {
                    let name_count = n.name().chars().count();
                    Slice {
                        line_start: n.line_start(),
                        fold: false,
                        origin: ctx.origin(),
                        source: n.source(),
                        annotations: vec![SourceAnnotation {
                            annotation_type: ctx.annotation_type(),
                            label: "space required here",
                            range: (name_count + 1, name_count + 2),
                        }],
                    }
                })
                .collect();

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some("preamble header values must begin with a space"),
                }),
                footer: vec![],
                slices,
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
