/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct OneOf<'n> {
    pub name: &'n str,
    pub values: &'n [&'n str],
}

impl<'n> Lint for OneOf<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name) {
            None => return Ok(()),
            Some(f) => f,
        };

        if self.values.contains(&field.value().trim()) {
            return Ok(());
        }

        let label = format!("preamble header `{}` has an unrecognized value", self.name);

        let slice_label = format!("must be one of: `{}`", self.values.join("`, `"));

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
