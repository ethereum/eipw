/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneOf<S> {
    pub name: S,
    pub values: Vec<S>,
}

impl<S> Lint for OneOf<S>
where
    S: Debug + Display + AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(f) => f,
        };

        let value = field.value().trim();

        if self.values.iter().any(|e| e == &value) {
            return Ok(());
        }

        let label = format!("preamble header `{}` has an unrecognized value", self.name);

        let values: Vec<_> = self.values.iter().map(|a| a.as_ref()).collect();
        let slice_label = format!("must be one of: `{}`", values.join("`, `"));

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
