/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Url<S>(pub S);

impl<S> Lint for Url<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
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

        let name_count = field.name().len();
        let value_count = field.value().len();

        ctx.report(
            ctx.annotation_level().title(&label).id(slug).snippet(
                Snippet::source(field.source())
                    .fold(false)
                    .line_start(field.line_start())
                    .origin_opt(ctx.origin())
                    .annotation(
                        ctx.annotation_level()
                            .span_utf8(field.source(), name_count + 1, value_count)
                            .label(&slice_label),
                    ),
            ),
        )?;

        Ok(())
    }
}
