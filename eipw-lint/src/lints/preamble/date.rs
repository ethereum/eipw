/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::Snippet;

use chrono::NaiveDate;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Date<S>(pub S);

impl<S> Lint for Date<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let value = field.value().trim();

        let mut error = None;

        let lengths: Vec<_> = value.split('-').map(str::len).collect();
        if lengths != [4, 2, 2] {
            error = Some("invalid length".to_string());
        }

        if let Err(e) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            error = Some(e.to_string());
        }

        let slice_label = match error {
            Some(e) => e,
            None => return Ok(()),
        };

        let label = format!(
            "preamble header `{}` is not a date in the `YYYY-MM-DD` format",
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
                            .label(&slice_label),
                    ),
            ),
        )?;

        Ok(())
    }
}
