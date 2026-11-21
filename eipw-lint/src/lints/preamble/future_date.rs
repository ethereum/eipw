/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use chrono::{Duration, NaiveDate, Utc};
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
pub struct FutureDate<S>(pub S);

impl<S> Lint for FutureDate<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(field) => field,
        };

        let value = field.value().trim();

        let deadline = match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            Ok(deadline) => deadline,
            Err(_) => return Ok(()),
        };

        let threshold = (Utc::now() - Duration::hours(12)).date_naive();

        if deadline >= threshold {
            return Ok(());
        }

        let label = format!(
            "preamble header `{}` must be today or a future date",
            self.0
        );

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
                            .label("date is in the past"),
                    ),
            ),
        )?;

        Ok(())
    }
}
