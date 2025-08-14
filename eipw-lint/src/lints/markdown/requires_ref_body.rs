/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::{Ast, AstNode, NodeCode, NodeCodeBlock, NodeHtmlBlock};
use eipw_snippets::Snippet;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::{LevelExt, SnippetExt};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct RequiresRefBody<S> {
    pub requires: S,
}

impl<S> RequiresRefBody<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn find_body_refs<'a>(&self, node: &'a AstNode<'a>) -> Result<HashSet<u32>, Error> {
        let mut visitor = Visitor::new();
        node.traverse().visit(&mut visitor)?;
        Ok(visitor.refs)
    }
}

impl<S> Lint for RequiresRefBody<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        // Get the requires field from preamble
        let requires_field = match ctx.preamble().by_name(self.requires.as_ref()) {
            None => return Ok(()), // No requires field, nothing to check
            Some(field) => field,
        };

        // Parse the required EIP numbers
        let required_eips: Vec<u32> = requires_field
            .value()
            .split(',')
            .map(str::trim)
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if required_eips.is_empty() {
            return Ok(()); // No valid EIP numbers in requires
        }

        // Find all EIP/ERC references in the body
        let body_refs = self.find_body_refs(ctx.body())?;

        // Find missing references
        let missing_eips: Vec<u32> = required_eips
            .into_iter()
            .filter(|eip| !body_refs.contains(eip))
            .collect();

        if missing_eips.is_empty() {
            return Ok(()); // All required EIPs are referenced in body
        }

        // Report error for missing EIPs
        let missing_str: Vec<String> = missing_eips.iter().map(|n| format!("EIP-{}", n)).collect();
        let label = format!(
            "proposals {} must be mentioned in the body",
            missing_str.join(", ")
        );

        let name_count = requires_field.name().len();
        let source = requires_field.source();
        let value_start = name_count + 2; // +2 for the colon and space

        ctx.report(
            ctx.annotation_level()
                .title(&label)
                .id(slug)
                .snippet(
                    Snippet::source(source)
                        .line_start(requires_field.line_start())
                        .origin_opt(ctx.origin())
                        .annotation(
                            ctx.annotation_level()
                                .span_utf8(source, value_start, requires_field.value().len())
                                .label("required here"),
                        )
                        .fold(false),
                ),
        )?;

        Ok(())
    }
}

struct Visitor {
    re: Regex,
    refs: HashSet<u32>,
}

impl Visitor {
    fn new() -> Self {
        Self {
            re: Regex::new(r"(?i)\b(?:eip|erc)-([0-9]+)\b").unwrap(),
            refs: Default::default(),
        }
    }
}

impl tree::Visitor for Visitor {
    type Error = Error;

    fn enter_front_matter(&mut self, _: &Ast, _: &str) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code(&mut self, _ast: &Ast, _code: &NodeCode) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code_block(&mut self, _: &Ast, _: &NodeCodeBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_inline(&mut self, _: &Ast, _: &str) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_block(&mut self, _: &Ast, _: &NodeHtmlBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_text(&mut self, _ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        for found in self.re.captures_iter(txt) {
            let number_txt = found.get(1).unwrap().as_str();
            if let Ok(number) = number_txt.parse::<u32>() {
                self.refs.insert(number);
            }
        }

        Ok(Next::TraverseChildren)
    }
}
