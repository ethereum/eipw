/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::{Ast, NodeCode, NodeCodeBlock, NodeHtmlBlock};
use eipw_snippets::Snippet;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::{LevelExt, SnippetExt};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct RequireReferenced<S> {
    pub requires: S,
}

impl<S> Lint for RequireReferenced<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires.as_ref()) {
            None => return Ok(()),
            Some(f) => f,
        };

        let mut visitor = Visitor::new();
        ctx.body().traverse().visit(&mut visitor)?;
        let referenced = visitor.referenced;

        let value = field.value();
        let name_count = field.name().len();

        let mut offset = 0;
        let mut missing = Vec::new();
        for matched in value.split(',') {
            let current = offset;
            offset += matched.len() + 1;

            let trimmed = matched.trim();
            let number: u64 = match trimmed.parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            if referenced.contains(&number) {
                continue;
            }

            let leading = matched.len() - matched.trim_start().len();
            let start = name_count + current + 1 + leading;
            missing.push((start, trimmed.len()));
        }

        if missing.is_empty() {
            return Ok(());
        }

        let label = format!(
            "proposals listed in preamble header `{}` must be referenced in the body",
            self.requires,
        );

        let annotations = missing.into_iter().map(|(start, len)| {
            ctx.annotation_level()
                .span_utf8(field.source(), start, len)
                .label("not referenced in body")
        });

        ctx.report(
            ctx.annotation_level().title(&label).id(slug).snippet(
                Snippet::source(field.source())
                    .annotations(annotations)
                    .fold(false)
                    .line_start(field.line_start())
                    .origin_opt(ctx.origin()),
            ),
        )?;

        Ok(())
    }
}

struct Visitor {
    re: Regex,
    referenced: HashSet<u64>,
}

impl Visitor {
    fn new() -> Self {
        Self {
            re: Regex::new(r"(?i)\b(?:eip|erc)-([0-9]+)\b").unwrap(),
            referenced: HashSet::new(),
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

    fn enter_text(&mut self, _: &Ast, txt: &str) -> Result<Next, Self::Error> {
        for found in self.re.captures_iter(txt) {
            let number_txt = found.get(1).unwrap().as_str();
            if let Ok(number) = number_txt.parse() {
                self.referenced.insert(number);
            }
        }

        Ok(Next::TraverseChildren)
    }
}
