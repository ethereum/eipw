/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeCode, NodeCodeBlock, NodeHtmlBlock, NodeLink};

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use ::regex::bytes::Regex as BytesRegex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum Mode {
    /// Ensure that each syntax node individually doesn't contain the pattern.
    Excludes,
    // TODO: Add includes/excludes modes that first renders to plain text, then
    //       matches the pattern.
}

#[derive(Debug)]
pub struct Regex<'n> {
    pub mode: Mode,
    pub pattern: &'n str,
    pub message: &'n str,
}

impl<'n> Lint for Regex<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let re = BytesRegex::new(self.pattern).map_err(Error::custom)?;

        let mut visitor = match self.mode {
            Mode::Excludes => ExcludesVisitor {
                ctx,
                re,
                message: self.message,
                pattern: self.pattern,
                slug,
            },
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct ExcludesVisitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    re: BytesRegex,
    pattern: &'c str,
    slug: &'c str,
    message: &'c str,
}

impl<'a, 'b, 'c> ExcludesVisitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast, buf: &[u8]) -> Result<Next, Error> {
        if !self.re.is_match(buf) {
            return Ok(Next::TraverseChildren);
        }

        let footer_label = format!("the pattern in question: `{}`", self.pattern);

        // TODO: Actually annotate the matches for `Mode::Excludes`.

        let source = self.ctx.source_for_text(ast.start_line, buf);
        self.ctx.report(Snippet {
            title: Some(Annotation {
                annotation_type: AnnotationType::Error,
                id: Some(self.slug),
                label: Some(self.message),
            }),
            slices: vec![Slice {
                fold: false,
                line_start: usize::try_from(ast.start_line).unwrap(),
                origin: self.ctx.origin(),
                source: &source,
                annotations: vec![],
            }],
            footer: vec![Annotation {
                id: None,
                annotation_type: AnnotationType::Info,
                label: Some(&footer_label),
            }],
            opt: Default::default(),
        })?;

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for ExcludesVisitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_front_matter(&mut self, _: &Ast, _: &[u8]) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code(&mut self, _ast: &Ast, _code: &NodeCode) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code_block(&mut self, _: &Ast, _: &NodeCodeBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_inline(&mut self, _: &Ast, _: &[u8]) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_block(&mut self, _: &Ast, _: &NodeHtmlBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_footnote_definition(&mut self, ast: &Ast, defn: &[u8]) -> Result<Next, Self::Error> {
        self.check(ast, defn)
    }

    fn enter_text(&mut self, ast: &Ast, txt: &[u8]) -> Result<Next, Self::Error> {
        self.check(ast, txt)
    }

    fn enter_link(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.check(ast, &link.title)
    }

    fn enter_image(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.check(ast, &link.title)
    }

    fn enter_footnote_reference(&mut self, ast: &Ast, refn: &[u8]) -> Result<Next, Self::Error> {
        self.check(ast, refn)
    }
}
