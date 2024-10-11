/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::Snippet;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Trim;

impl Lint for Trim {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let mut no_space = Vec::new();

        for field in ctx.preamble().fields() {
            let mut value = field.value();
            if value.is_empty() {
                continue;
            }

            if let Some(v) = value.strip_prefix(' ') {
                value = v;
            } else {
                no_space.push(field);
            }

            if value.trim() == value {
                continue;
            }

            let name_count = field.name().len();
            let value_count = field.value().len();

            let label = format!("preamble header `{}` has extra whitespace", field.name());
            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(field.source())
                        .fold(false)
                        .origin_opt(ctx.origin())
                        .line_start(field.line_start())
                        .annotation(
                            ctx.annotation_level()
                                .span_utf8(field.source(), name_count + 1, value_count)
                                .label("value has extra whitespace"),
                        ),
                ),
            )?;
        }

        if !no_space.is_empty() {
            let slices = no_space.into_iter().map(|n| {
                let name_count = n.name().len();
                Snippet::source(n.source())
                    .line_start(n.line_start())
                    .fold(false)
                    .origin_opt(ctx.origin())
                    .annotation(
                        ctx.annotation_level()
                            .span_utf8(n.source(), name_count + 1, 1)
                            .label("space required here"),
                    )
            });

            ctx.report(
                ctx.annotation_level()
                    .title("preamble header values must begin with a space")
                    .id(slug)
                    .snippets(slices),
            )?;
        }

        Ok(())
    }
}
