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

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RequiredIfEq<S> {
    pub when: S,
    pub equals: S,
    pub then: S,
}

impl<S> Lint for RequiredIfEq<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let then_opt = ctx.preamble().by_name(self.then.as_ref());
        let when_opt = ctx.preamble().by_name(self.when.as_ref());

        let equals = self.equals.as_ref();

        match (when_opt, then_opt) {
            // Correct.
            (None, None) => (),

            // Correct.
            (Some(when), Some(_)) if when.value().trim() == equals => (),

            // Correct.
            (Some(when), None) if when.value().trim() != equals => (),

            // Incorrect.
            (Some(when), None) => {
                let label = format!(
                    "preamble header `{}` is required when `{}` is `{}`",
                    self.then, self.when, self.equals,
                );
                ctx.report(
                    ctx.annotation_level().title(&label).id(slug).snippet(
                        Snippet::source(when.source())
                            .line_start(when.line_start())
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .annotation(
                                Level::Info
                                    .span(0..when.source().len())
                                    .label("defined here"),
                            ),
                    ),
                )?;
            }

            // Incorrect.
            (Some(when), Some(then)) => {
                let label = format!(
                    "preamble header `{}` is only allowed when `{}` is `{}`",
                    self.then, self.when, self.equals,
                );

                let info_label = format!("unless equal to `{}`", self.equals);

                let mut slices = vec![
                    (
                        when.line_start(),
                        Snippet::source(when.source())
                            .line_start(when.line_start())
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .annotation(
                                Level::Info.span(0..when.source().len()).label(&info_label),
                            ),
                    ),
                    (
                        then.line_start(),
                        Snippet::source(then.source())
                            .line_start(then.line_start())
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .annotation(
                                ctx.annotation_level()
                                    .span(0..then.source().len())
                                    .label("remove this"),
                            ),
                    ),
                ];

                slices.sort_by_key(|(line_start, _)| *line_start);

                ctx.report(
                    ctx.annotation_level()
                        .title(&label)
                        .id(slug)
                        .snippets(slices.into_iter().map(|(_, s)| s)),
                )?;
            }

            // Incorrect.
            (None, Some(then)) => {
                let label = format!(
                    "preamble header `{}` is only allowed when `{}` is `{}`",
                    self.then, self.when, self.equals,
                );

                ctx.report(
                    ctx.annotation_level().title(&label).id(slug).snippet(
                        Snippet::source(then.source())
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .line_start(then.line_start())
                            .annotation(
                                ctx.annotation_level()
                                    .span(0..then.source().len())
                                    .label("defined here"),
                            ),
                    ),
                )?;
            }
        }

        Ok(())
    }
}
