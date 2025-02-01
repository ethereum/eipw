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
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct List<S>(pub S);

impl<S> Lint for List<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let mut missing_space = Vec::new();
        let mut extra_space = Vec::new();

        let value = field.value().trim();
        if value.is_empty() {
            return Ok(());
        }

        let mut offset = 0;
        for matched in value.split(',') {
            let current = offset;
            offset += matched.len() + 1;

            let name_count = field.name().len();

            let trimmed = matched.trim();
            if trimmed.is_empty() {
                let label = format!("preamble header `{}` cannot have empty items", self.0);
                ctx.report(
                    ctx.annotation_level().title(&label).id(slug).snippet(
                        Snippet::source(field.source())
                            .fold(false)
                            .line_start(field.line_start())
                            .origin_opt(ctx.origin())
                            .annotation(
                                ctx.annotation_level()
                                    .span_utf8(field.source(), name_count + current + 1, 1)
                                    .label("this item is empty"),
                            ),
                    ),
                )?;
                continue;
            }

            let rest = match matched.strip_prefix(' ') {
                Some(r) => r,
                None if current == 0 => matched,
                None => {
                    let start = name_count + current + 1;
                    missing_space.push(
                        ctx.annotation_level()
                            .span_utf8(field.source(), start, 1)
                            .label("missing space"),
                    );
                    continue;
                }
            };

            if rest.trim() == rest {
                continue;
            }

            let start = name_count + current + 2;
            extra_space.push(
                ctx.annotation_level()
                    .span_utf8(field.source(), start, matched.len())
                    .label("extra space"),
            );
        }

        if !missing_space.is_empty() {
            ctx.report(
                ctx.annotation_level()
                    .title("preamble header list items must begin with a space")
                    .id(slug)
                    .snippet(
                        Snippet::source(field.source())
                            .line_start(field.line_start())
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .annotations(missing_space),
                    ),
            )?;
        }

        if !extra_space.is_empty() {
            ctx.report(
                ctx.annotation_level()
                    .title("preamble header list items have extra whitespace")
                    .id(slug)
                    .snippet(
                        Snippet::source(field.source())
                            .line_start(field.line_start())
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .annotations(extra_space),
                    ),
            )?;
        }

        Ok(())
    }
}
