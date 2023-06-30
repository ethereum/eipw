/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Required<S>(pub Vec<S>);

impl<S> Lint for Required<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let missing = self
            .0
            .iter()
            .map(AsRef::as_ref)
            .filter(|name| ctx.preamble().by_name(name).is_none())
            .collect::<Vec<_>>()
            .join("`, `");

        if !missing.is_empty() {
            let label = format!("preamble is missing header(s): `{}`", missing);
            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: ctx.annotation_type(),
                    label: Some(&label),
                }),
                footer: vec![],
                slices: vec![Slice {
                    fold: true,
                    annotations: vec![],
                    line_start: 1,
                    source: ctx.line(1),
                    origin: ctx.origin(),
                }],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
