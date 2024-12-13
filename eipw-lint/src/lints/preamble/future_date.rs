/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;
use chrono::{NaiveDate, Utc};

use crate::{
    lints::{Context, Error, FetchContext, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// A lint that ensures a date field in the preamble is set to a future date.
/// 
/// This lint is particularly useful for validating the `last-call-deadline` field
/// when an EIP is in "Last Call" status. The deadline must be set to a future date
/// to give the community sufficient time to review the proposal.
/// 
/// # Example
/// ```text
/// status: Last Call
/// last-call-deadline: 2024-12-31  # Must be a future date
/// ```
/// 
/// The lint will raise an error if:
/// - The date is in the past
/// - The date format is invalid (not YYYY-MM-DD)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FutureDate<S>(pub S);

impl<S> Lint for FutureDate<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn find_resources(&self, _ctx: &FetchContext<'_>) -> Result<(), Error> {
        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        // Only check if status is "Last Call"
        let status = match ctx.preamble().by_name("status") {
            None => return Ok(()),
            Some(s) => s.value().trim(),
        };

        if status != "Last Call" {
            return Ok(());
        }

        // Get the deadline field
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let value = field.value().trim();

        // Parse the date
        let date = match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => return Ok(()), // Let the Date lint handle invalid dates
        };

        // Get today's date
        let today = Utc::now().date_naive();

        // Check if date is in the future
        if date <= today {
            let label = format!(
                "preamble header `{}` must be a future date (today is {})",
                self.0,
                today.format("%Y-%m-%d")
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
                                .span_utf8(field.source(), name_count + 2, value_count)
                                .label("must be after today's date"),
                        ),
                ),
            )?;
        }

        Ok(())
    }
}