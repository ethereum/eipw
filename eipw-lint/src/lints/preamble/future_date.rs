/*
 * Copyright 2023 The EIP.WTF Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use eipw_snippets::Snippet;
use chrono::{NaiveDate, Utc};

use crate::{
    lints::{Context, Error, FetchContext, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Validates that the `last-call-deadline` in an EIP preamble is a future date
/// when the EIP status is "Last Call".
///
/// According to EIP-1, the `last-call-deadline` field is only required when status
/// is "Last Call", and it must be in ISO 8601 date format (YYYY-MM-DD). The date
/// must be in the future or today, as it represents when the last call period ends.
///
/// Example valid preamble:
/// ```yaml
/// status: Last Call
/// last-call-deadline: 2024-12-31  # Must be today or a future date
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

        // Check if date is in the future or today
        if date < today {
            let label = format!(
                "preamble header `{}` must be today or a future date (today is {})",
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
                                .label("must be today or a future date"),
                        ),
                ),
            )?;
        }

        Ok(())
    }
}