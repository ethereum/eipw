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
pub struct Uint<S>(pub S);

impl<S> Lint for Uint<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        if field.value().trim().parse::<u64>().is_err() {
            let name_count = field.name().chars().count();
            let value_count = field.value().chars().count();

            let label = format!("preamble header `{}` must be an unsigned integer", self.0);

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
                        label: "not a non-negative integer",
                        range: (name_count + 1, value_count + name_count + 1),
                    }],
                }],
                footer: vec![],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct UintList<S>(pub S);

impl<S> Lint for UintList<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        if field.value().trim().is_empty() {
            return Ok(());
        }

        let items = field.value().split(','); // Don't trim here so the offsets line up later.
        let mut values: Vec<u64> = Vec::new();
        let mut not_uint = Vec::new();

        let name_count = field.name().chars().count();

        let mut offset = 0;

        for item in items {
            let item_count = item.chars().count();

            let current = offset;
            offset += item_count + 1;
            let trimmed = item.trim();

            match trimmed.parse() {
                Ok(v) => values.push(v),
                Err(_) => {
                    not_uint.push(SourceAnnotation {
                        annotation_type: ctx.annotation_type(),
                        label: "not a non-negative integer",
                        range: (
                            name_count + current + 1,
                            name_count + current + 1 + item_count,
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
                    annotation_type: ctx.annotation_type(),
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
                    annotation_type: ctx.annotation_type(),
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
