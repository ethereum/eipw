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
pub struct Length<S> {
    pub name: S,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl<S> Lint for Length<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(f) => f,
        };

        let value = field.value().trim();

        let name_count = field.name().chars().count();
        let value_count = field.value().chars().count();

        if let Some(max) = self.max {
            if value.len() > max {
                let label = format!(
                    "preamble header `{}` value is too long (max {})",
                    self.name, max,
                );

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
                            label: "too long",
                            range: (name_count + 1, value_count + name_count + 1),
                        }],
                    }],
                    opt: Default::default(),
                })?;
            }
        }

        if let Some(min) = self.min {
            if value.len() < min {
                let label = format!(
                    "preamble header `{}` value is too short (min {})",
                    self.name, min,
                );

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
                            label: "too short",
                            range: (name_count + 1, value_count + name_count + 1),
                        }],
                    }],
                    opt: Default::default(),
                })?;
            }
        }

        Ok(())
    }
}
