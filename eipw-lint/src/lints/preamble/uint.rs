/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct Uint<'n>(pub &'n str);

impl<'n> Lint for Uint<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0) {
            None => return Ok(()),
            Some(s) => s,
        };

        if field.value().trim().parse::<u64>().is_err() {
            let label = format!("preamble header `{}` must be an unsigned integer", self.0);

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    line_start: field.line_start(),
                    fold: false,
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: vec![SourceAnnotation {
                        annotation_type: AnnotationType::Error,
                        label: "not a non-negative integer",
                        range: (
                            field.name().len() + 1,
                            field.value().len() + field.name().len() + 1,
                        ),
                    }],
                }],
                footer: vec![],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct UintList<'n>(pub &'n str);

impl<'n> Lint for UintList<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0) {
            None => return Ok(()),
            Some(s) => s,
        };

        let items = field.value().split(',');
        let mut values: Vec<u64> = Vec::new();
        let mut not_uint = Vec::new();

        let mut offset = 0;

        for item in items {
            let current = offset;
            offset += item.len() + 1;
            let trimmed = item.trim();

            match trimmed.parse() {
                Ok(v) => values.push(v),
                Err(_) => {
                    not_uint.push(SourceAnnotation {
                        annotation_type: AnnotationType::Error,
                        label: "not a non-negative integer",
                        range: (
                            field.name().len() + current + 1,
                            field.name().len() + current + 1 + item.len(),
                        ),
                    });
                    continue;
                }
            }
        }

        if !not_uint.is_empty() {
            let label = format!(
                "preamble header `{}` items must be unsigned integers",
                self.0
            );

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: not_uint,
                }],
                footer: vec![],
                opt: Default::default(),
            })?;
        }

        // TODO: replace with `is_sorted` when #53485 is stabilized
        let mut sorted = values.clone();
        sorted.sort_unstable();

        if sorted != values {
            let label = format!(
                "preamble header `{}` items must be sorted in ascending order",
                self.0
            );

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: vec![],
                }],
                footer: vec![],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
