/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet};

use annotate_snippets::snippet::SourceAnnotation;
use comrak::nodes::Ast;
use comrak::nodes::NodeValue;
use regex::Regex;

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeadingsSpace;

impl Lint for HeadingsSpace {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        // Match for text nodes starting with with 1 to 6 '#' chars 
        // as Markdown does not recognise headings without space
        let heading_pattern = Regex::new("^#{1,6}").unwrap();
        let false_headings: Vec<_> = ctx
            .body()
            .descendants()
            .filter_map(|node| match &*node.data.borrow() {
                // Collect all matching Text nodes 
                Ast {
                    value: NodeValue::Text(text),
                    ..
                } => {
                    if let Some(matched_text) = heading_pattern.find(text) {
                        let heading_level = matched_text.len();
                        Some((text.clone(), node.data.borrow().sourcepos.start.line, heading_level))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        let slices = false_headings
            .iter()
            .map(|(text, line_start, heading_level)| {
                Slice {
                    line_start: line_start.clone(),
                    origin: ctx.origin(),
                    source: text,
                    fold: false,
                    annotations: vec![SourceAnnotation {
                        annotation_type: ctx.annotation_type(),
                        label: "space required here",
                        range: (*heading_level - 1, *heading_level),
                    }],
                }
            })
            .collect();

        ctx.report(Snippet {
            title: Some(Annotation {
                id: Some(slug),
                annotation_type: ctx.annotation_type(),
                label: Some("Space missing in header"),
            }),
            footer: vec![],
            slices,
            opt: Default::default(),
        })?;
        Ok(())
    }
}
