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
pub struct Length<S> {
    pub name: S,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl<S> Lint for Length<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(f) => f,
        };

        let value = field.value().trim();

        let name_count = field.name().len();
        let value_count = field.value().len();

        if let Some(max) = self.max {
            if value.len() > max {
                let label = format!(
                    "preamble header `{}` value is too long (max {})",
                    self.name, max,
                );

                ctx.report(
                    ctx.annotation_level().title(&label).id(slug).snippet(
                        Snippet::source(field.source())
                            .fold(false)
                            .line_start(field.line_start())
                            .origin_opt(ctx.origin())
                            .annotation(
                                ctx.annotation_level()
                                    .span_utf8(field.source(), name_count + 1, value_count)
                                    .label("too long"),
                            ),
                    ),
                )?;
            }
        }

        if let Some(min) = self.min {
            if value.len() < min {
                let label = format!(
                    "preamble header `{}` value is too short (min {})",
                    self.name, min,
                );

                ctx.report(
                    ctx.annotation_level().title(&label).id(slug).snippet(
                        Snippet::source(field.source())
                            .fold(false)
                            .line_start(field.line_start())
                            .origin_opt(ctx.origin())
                            .annotation(
                                ctx.annotation_level()
                                    .span_utf8(field.source(), name_count + 1, value_count)
                                    .label("too short"),
                            ),
                    ),
                )?;
            }
        }

        Ok(())
    }
}
