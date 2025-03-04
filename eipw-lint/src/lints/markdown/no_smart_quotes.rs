/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Snippet};

use comrak::nodes::{
    Ast, NodeCode, NodeCodeBlock, NodeFootnoteDefinition, NodeFootnoteReference, NodeHtmlBlock,
    NodeLink,
};

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::SnippetExt;

use ::regex::Regex;

use serde::{Deserialize, Serialize};

use std::fmt::Debug;

// Smart quotes to detect: " " ' ' - using Unicode code points and a fixed regex pattern
// for clarity in the source code
const SMART_QUOTES_PATTERN: &str = "[\u{201C}\u{201D}]|[\u{2018}\u{2019}]";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NoSmartQuotes;

impl Lint for NoSmartQuotes {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let re = Regex::new(SMART_QUOTES_PATTERN).map_err(Error::custom)?;
        let mut visitor = Visitor {
            ctx,
            re,
            slug,
        };
        ctx.body().traverse().visit(&mut visitor)?;
        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    re: Regex,
    slug: &'c str,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast, text: &str) -> Result<Next, Error> {
        if !self.re.is_match(text) {
            return Ok(Next::TraverseChildren);
        }

        let footer_label = "Smart quotes detected: replace with straight quotes (\", ')";
        let source = self.ctx.source_for_text(ast.sourcepos.start.line, text);
        self.ctx.report(
            self.ctx
                .annotation_level()
                .title("smart quotes are not allowed (use straight quotes instead)")
                .id(self.slug)
                .snippet(
                    Snippet::source(&source)
                        .fold(false)
                        .line_start(ast.sourcepos.start.line)
                        .origin_opt(self.ctx.origin()),
                )
                .footer(Level::Info.title(footer_label)),
        )?;

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_front_matter(&mut self, _: &Ast, _: &str) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code(&mut self, ast: &Ast, code: &NodeCode) -> Result<Next, Self::Error> {
        // Check code blocks for smart quotes which could be especially problematic
        self.check(ast, &code.literal)
    }

    fn enter_code_block(&mut self, ast: &Ast, block: &NodeCodeBlock) -> Result<Next, Self::Error> {
        // Check code blocks for smart quotes which could be especially problematic
        self.check(ast, &block.literal)
    }

    fn enter_html_inline(&mut self, _: &Ast, _: &str) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_block(&mut self, _: &Ast, _: &NodeHtmlBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_footnote_definition(
        &mut self,
        ast: &Ast,
        defn: &NodeFootnoteDefinition,
    ) -> Result<Next, Self::Error> {
        self.check(ast, &defn.name)
    }

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        self.check(ast, txt)
    }

    fn enter_link(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.check(ast, &link.title)
    }

    fn enter_image(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.check(ast, &link.title)
    }

    fn enter_footnote_reference(
        &mut self,
        ast: &Ast,
        refn: &NodeFootnoteReference,
    ) -> Result<Next, Self::Error> {
        self.check(ast, &refn.name)
    }
} 