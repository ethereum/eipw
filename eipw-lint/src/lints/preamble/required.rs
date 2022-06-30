/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Snippet};

use crate::lints::{Context, Error, Lint};

#[derive(Debug)]
pub struct Required<'n>(pub &'n [&'n str]);

impl<'n> Lint for Required<'n> {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a>) -> Result<(), Error> {
        let missing = self
            .0
            .iter()
            .filter(|name| ctx.preamble().by_name(name).is_none())
            .cloned()
            .collect::<Vec<_>>()
            .join("`, `");

        if !missing.is_empty() {
            let label = format!("preamble is missing header(s): `{}`", missing);
            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: AnnotationType::Error,
                    label: Some(&label),
                }),
                footer: vec![],
                slices: vec![],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
