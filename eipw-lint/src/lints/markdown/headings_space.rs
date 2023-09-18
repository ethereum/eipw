/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet};

use comrak::nodes::Ast;
use comrak::nodes::NodeValue;

use crate::lints::{Context, Error, Lint};
use crate::reporters::text;
use crate::tree::{self, Next, TraverseExt};

use regex::{Regex, RegexSet};

use scraper::node::Node as HtmlNode;
use scraper::Html;

use serde::{Deserialize, Serialize};

use snafu::Snafu;

use std::fmt::{Debug, Display};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeadingsSpace;

impl Lint for HeadingsSpace {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        // Collect all headings.
        let false_headings: Vec<_> = ctx
        .body()
        .descendants()
        .filter_map(|node| match &*node.data.borrow() { 
            Ast { value: NodeValue::Text(text), .. } => {
                if text.starts_with("#") {
                    Some((text.clone(), node.data.borrow().sourcepos.start.line))
                }
                else {
                    None
                }
            },
            _ => None,
        }).collect();

        let slices = false_headings.iter()
        .map(|(text, line_start)| {
            Slice{
                line_start: line_start.clone(),
                origin: ctx.origin(),
                source: text,
                fold: false,
                annotations: vec![],
            }
        })
        .collect();

        // Print all false headings
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
