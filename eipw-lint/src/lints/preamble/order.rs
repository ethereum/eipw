/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct Order<'n>(pub &'n [&'n str]);

impl<'n> Lint for Order<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        // Check for unknown headers.
        let unknowns: Vec<_> = ctx
            .preamble()
            .fields()
            .filter(|f| !self.0.contains(&f.name()))
            .map(|f| Slice {
                line_start: f.line_start(),
                fold: false,
                origin: ctx.origin(),
                source: f.source(),
                annotations: vec![SourceAnnotation {
                    annotation_type: AnnotationType::Error,
                    label: "unrecognized header",
                    range: (0, f.name().len()),
                }],
            })
            .collect();

        if !unknowns.is_empty() {
            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: AnnotationType::Error,
                    label: Some("preamble has extra header(s)"),
                }),
                footer: vec![],
                slices: unknowns,
                opt: Default::default(),
            })?;
        }

        // Check that headers are in the correct order.
        let mut max_line = 0;
        for (idx, name) in self.0.iter().enumerate() {
            if let Some(field) = ctx.preamble().by_name(name) {
                let cur = max_line;
                max_line = field.line_start();

                if max_line >= cur {
                    continue;
                }

                let label = format!(
                    "preamble header `{}` must come after `{}`",
                    field.name(),
                    self.0[idx - 1]
                );
                ctx.report(Snippet {
                    title: Some(Annotation {
                        id: Some(slug),
                        annotation_type: AnnotationType::Error,
                        label: Some(&label),
                    }),
                    footer: vec![],
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
