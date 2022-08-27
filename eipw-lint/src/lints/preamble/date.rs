/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use chrono::NaiveDate;

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct Date<'n>(pub &'n str);

impl<'n> Lint for Date<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0) {
            None => return Ok(()),
            Some(s) => s,
        };

        let value = field.value().trim();

        let mut error = None;

        let lengths: Vec<_> = value.split('-').map(str::len).collect();
        if lengths != [4, 2, 2] {
            error = Some("invalid length".to_string());
        }

        if let Err(e) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            error = Some(e.to_string());
        }

        let slice_label = match error {
            Some(e) => e,
            None => return Ok(()),
        };

        let label = format!(
            "preamble header `{}` is not a date in the `YYYY-MM-DD` format",
            self.0
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
                    label: &slice_label,
                    range: (
                        field.name().len() + 1,
                        field.value().len() + field.name().len() + 1,
                    ),
                }],
            }],
            opt: Default::default(),
        })?;

        Ok(())
    }
}
