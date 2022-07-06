/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct Length<'n> {
    pub name: &'n str,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl<'n> Lint for Length<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name) {
            None => return Ok(()),
            Some(f) => f,
        };

        let value = field.value().trim();

        if let Some(max) = self.max {
            if value.len() > max {
                let label = format!(
                    "preamble header `{}` value is too long (max {})",
                    self.name, max,
                );

                ctx.report(Snippet {
                    title: Some(Annotation {
                        annotation_type: AnnotationType::Error,
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
                            annotation_type: AnnotationType::Error,
                            label: "too long",
                            range: (
                                field.name().len() + 1,
                                field.value().len() + field.name().len() + 1,
                            ),
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
                        annotation_type: AnnotationType::Error,
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
                            annotation_type: AnnotationType::Error,
                            label: "too short",
                            range: (
                                field.name().len() + 1,
                                field.value().len() + field.name().len() + 1,
                            ),
                        }],
                    }],
                    opt: Default::default(),
                })?;
            }
        }

        Ok(())
    }
}
