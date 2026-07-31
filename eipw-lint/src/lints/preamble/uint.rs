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
pub struct Uint<S>(pub S);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct ConfiguredUint<S> {
    pub name: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<UintMessage<S>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct UintMessage<S> {
    pub value: S,
    pub message: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<S>,
}

impl<S> ConfiguredUint<S> {
    pub fn new(name: S) -> Self {
        Self {
            name,
            messages: None,
        }
    }
}

impl<S> Lint for Uint<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        if field.value().trim().parse::<u64>().is_err() {
            let name_count = field.name().len();
            let value_count = field.value().len();

            let label = format!("preamble header `{}` must be an unsigned integer", self.0);

            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(field.source())
                        .line_start(field.line_start())
                        .fold(false)
                        .origin_opt(ctx.origin())
                        .annotation(
                            ctx.annotation_level()
                                .span_utf8(field.source(), name_count + 1, value_count)
                                .label("not a non-negative integer"),
                        ),
                ),
            )?;
        }

        Ok(())
    }
}

impl<S> Lint for ConfiguredUint<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let value = field.value().trim();
        if value.parse::<u64>().is_ok() {
            return Ok(());
        }

        let name_count = field.name().len();
        let value_count = field.value().len();

        let message = self
            .messages
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .find(|message| message.value.as_ref() == value);

        let label = message.map_or_else(
            || {
                format!(
                    "preamble header `{}` must be an unsigned integer",
                    self.name
                )
            },
            |message| message.message.as_ref().to_string(),
        );
        let span_label = message
            .and_then(|message| message.label.as_ref().map(AsRef::as_ref))
            .unwrap_or("not a non-negative integer");

        ctx.report(
            ctx.annotation_level().title(&label).id(slug).snippet(
                Snippet::source(field.source())
                    .line_start(field.line_start())
                    .fold(false)
                    .origin_opt(ctx.origin())
                    .annotation(
                        ctx.annotation_level()
                            .span_utf8(field.source(), name_count + 1, value_count)
                            .label(span_label),
                    ),
            ),
        )?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct UintList<S>(pub S);

impl<S> Lint for UintList<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        if field.value().trim().is_empty() {
            return Ok(());
        }

        let items = field.value().split(','); // Don't trim here so the offsets line up later.
        let mut values: Vec<u64> = Vec::new();
        let mut not_uint = Vec::new();

        let name_count = field.name().len();

        let mut offset = 0;

        for item in items {
            let item_count = item.len();

            let current = offset;
            offset += item_count + 1;
            let trimmed = item.trim();

            match trimmed.parse() {
                Ok(v) => values.push(v),
                Err(_) => {
                    let start = name_count + current + 1;
                    not_uint.push(
                        ctx.annotation_level()
                            .span_utf8(field.source(), start, item_count)
                            .label("not a non-negative integer"),
                    );
                    continue;
                }
            }
        }

        if !not_uint.is_empty() {
            let label = format!(
                "preamble header `{}` items must be unsigned integers",
                self.0
            );

            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(field.source())
                        .origin_opt(ctx.origin())
                        .fold(false)
                        .line_start(field.line_start())
                        .annotations(not_uint),
                ),
            )?;
        }

        // TODO: replace with `is_sorted` when #53485 is stabilized
        let mut sorted = values.clone();
        sorted.sort_unstable();

        if sorted != values {
            let label = format!(
                "preamble header `{}` items must be sorted in ascending order",
                self.0
            );

            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(field.source())
                        .line_start(field.line_start())
                        .fold(false)
                        .origin_opt(ctx.origin()),
                ),
            )?;
        }

        Ok(())
    }
}
