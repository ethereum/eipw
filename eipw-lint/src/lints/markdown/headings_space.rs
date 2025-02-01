/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use comrak::nodes::{Ast, LineColumn, NodeValue, Sourcepos};
use regex::Regex;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct HeadingsSpace;

impl Lint for HeadingsSpace {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        // Match for text nodes starting with leading '#' chars (upto 6)
        // Markdown does not recognise these nodes as valid Headings without the space
        let heading_pattern = Regex::new("^#{1,6}").unwrap();
        let invalid_headings: Vec<_> = ctx
            .body()
            .descendants()
            .filter_map(|node| match &*node.data.borrow() {
                // Collect all matching Text nodes
                Ast {
                    value: NodeValue::Text(text),
                    sourcepos:
                        Sourcepos {
                            start: LineColumn { column: 1, .. }, // Only match text nodes at the start of the line
                            ..
                        },
                    ..
                } => {
                    if let Some(matched_text) = heading_pattern.find(text) {
                        let heading_level = matched_text.end();
                        Some((
                            text.clone(),
                            node.data.borrow().sourcepos.start.line,
                            heading_level,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        let slices: Vec<_> = invalid_headings
            .iter()
            .map(|(text, line_start, heading_level)| {
                Snippet::source(text)
                    .line_start(*line_start)
                    .fold(false)
                    .origin_opt(ctx.origin())
                    .annotation(
                        ctx.annotation_level()
                            .span_utf8(text, heading_level - 1, 1)
                            .label("space required here"),
                    )
            })
            .collect();

        if !slices.is_empty() {
            ctx.report(
                ctx.annotation_level()
                    .title("Space missing in header")
                    .id(slug)
                    .snippets(slices),
            )?;
        }

        Ok(())
    }
}
