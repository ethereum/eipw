/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::{Level, Snippet};

use comrak::nodes::{Ast, NodeCode, NodeCodeBlock, NodeHtmlBlock, NodeLink};

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::SnippetExt;

use ::regex::Regex as TextRegex;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// Ensure that each syntax node individually doesn't contain the pattern.
    Excludes,
    // TODO: Add includes/excludes modes that first renders to plain text, then
    //       matches the pattern.
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Regex<S> {
    pub mode: Mode,
    pub pattern: S,
    pub message: S,
}

impl<S> Lint for Regex<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let pattern = self.pattern.as_ref();
        let re = TextRegex::new(pattern).map_err(Error::custom)?;

        let mut visitor = match self.mode {
            Mode::Excludes => ExcludesVisitor {
                ctx,
                re,
                message: self.message.as_ref(),
                pattern,
                slug,
            },
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct ExcludesVisitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    re: TextRegex,
    pattern: &'c str,
    slug: &'c str,
    message: &'c str,
}

impl<'a, 'b, 'c> ExcludesVisitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast, buf: &str) -> Result<Next, Error> {
        if !self.re.is_match(buf) {
            return Ok(Next::TraverseChildren);
        }

        let footer_label = format!("the pattern in question: `{}`", self.pattern);

        // TODO: Actually annotate the matches for `Mode::Excludes`.

        let source = self.ctx.source_for_text(ast.sourcepos.start.line, buf);
        self.ctx.report(
            self.ctx
                .annotation_level()
                .title(self.message)
                .id(self.slug)
                .snippet(
                    Snippet::source(&source)
                        .origin_opt(self.ctx.origin())
                        .line_start(ast.sourcepos.start.line)
                        .fold(false),
                )
                .footer(Level::Info.title(&footer_label)),
        )?;

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for ExcludesVisitor<'a, 'b, 'c> {
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

    fn enter_footnote_definition(&mut self, ast: &Ast, defn: &str) -> Result<Next, Self::Error> {
        self.check(ast, defn)
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

    fn enter_footnote_reference(&mut self, ast: &Ast, refn: &str) -> Result<Next, Self::Error> {
        self.check(ast, refn)
    }
}
