/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::{Level, Snippet};

use crate::{
    lints::{Context, Error, Lint},
    SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct NoDuplicates;

impl Lint for NoDuplicates {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let mut defined = HashMap::new();

        for field in ctx.preamble().fields() {
            match defined.entry(field.name()) {
                Entry::Vacant(v) => {
                    v.insert(field);
                }
                Entry::Occupied(o) => {
                    let original = o.get();
                    let original_count = original.source().len();
                    let field_count = field.source().len();
                    let label = format!(
                        "preamble header `{}` defined multiple times",
                        original.name()
                    );

                    ctx.report(
                        ctx.annotation_level()
                            .title(&label)
                            .id(slug)
                            .snippet(
                                Snippet::source(original.source())
                                    .line_start(original.line_start())
                                    .fold(false)
                                    .origin_opt(ctx.origin())
                                    .annotation(
                                        Level::Info
                                            .span(0..original_count)
                                            .label("first defined here"),
                                    ),
                            )
                            .snippet(
                                Snippet::source(field.source())
                                    .line_start(field.line_start())
                                    .origin_opt(ctx.origin())
                                    .fold(false)
                                    .annotation(
                                        ctx.annotation_level()
                                            .span(0..field_count)
                                            .label("redefined here"),
                                    ),
                            ),
                    )?;
                }
            }
        }

        Ok(())
    }
}
