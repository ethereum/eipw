/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use crate::{
    lints::{Context, Error, Lint},
    SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Required<S>(pub Vec<S>);

impl<S> Lint for Required<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let missing = self
            .0
            .iter()
            .map(AsRef::as_ref)
            .filter(|name| ctx.preamble().by_name(name).is_none())
            .collect::<Vec<_>>()
            .join("`, `");

        if !missing.is_empty() {
            let label = format!("preamble is missing header(s): `{}`", missing);
            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source("---")
                        .line_start(1)
                        .origin_opt(ctx.origin())
                        .fold(true),
                ),
            )?;
        }

        Ok(())
    }
}
