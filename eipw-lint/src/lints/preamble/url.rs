/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct Url<'n>(pub &'n str);

impl<'n> Lint for Url<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0) {
            Some(f) => f,
            None => return Ok(()),
        };

        let value = field.value().trim();

        let e = match ::url::Url::parse(value) {
            Ok(_) => return Ok(()),
            Err(e) => e,
        };

        let label = format!("preamble header `{}` is not a valid URL", self.0);
        let slice_label = e.to_string();

        let name_count = field.name().chars().count();
        let value_count = field.value().chars().count();

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
                    label: &slice_label,
                    range: (name_count + 1, value_count + name_count + 1),
                }],
            }],
            opt: Default::default(),
        })?;

        Ok(())
    }
}
