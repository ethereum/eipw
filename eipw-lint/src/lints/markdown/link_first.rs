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

use ::regex::Regex;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LinkFirst<S>(pub S);

impl<S> Lint for LinkFirst<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let pattern = self.0.as_ref();
        let re = Regex::new(pattern).map_err(Error::custom)?;

        let own_number = ctx
            .preamble()
            .by_name("eip")
            .map(|field| field.value().trim())
            .map(str::parse)
            .and_then(Result::ok);

        let mut visitor = Visitor {
            ctx,
            re,
            pattern,
            slug,
            own_number,
            linked: Default::default(),
            link_depth: 0,
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    re: Regex,
    pattern: &'c str,
    slug: &'c str,
    linked: HashSet<String>,
    link_depth: usize,
    own_number: Option<u32>,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast, text: &str) -> Result<Next, Error> {
        let source = self.ctx.ast_lines(ast);
        let offset = source.find(text); // TODO: Calculating the offset like this is a huge hack.

        for matched in self.re.captures_iter(text) {
            let self_reference = match self.own_number {
                None => false,
                Some(own_number) => matched
                    .get(1)
                    .expect("missing capture group for `LinkFirst` regex")
                    .as_str()
                    .parse()
                    .map(|n: u32| n == own_number)
                    .unwrap_or(false),
            };

            if self_reference {
                continue;
            }

            let matched_str = matched.get(0).unwrap().as_str();

            if self.linked.contains(matched_str) {
                continue;
            }

            let footer_label = format!("the pattern in question: `{}`", self.pattern);

            let annotations = match offset {
                None => None,
                Some(offset) => {
                    let start = offset + matched.get(0).unwrap().start();
                    let end = offset + matched.get(0).unwrap().end();
                    Some(self.ctx.annotation_level().span(start..end))
                }
            };

            self.ctx.report(
                self.ctx
                    .annotation_level()
                    .title("the first match of the given pattern must be a link")
                    .id(self.slug)
                    .snippet(
                        Snippet::source(source)
                            .fold(true)
                            .line_start(ast.sourcepos.start.line)
                            .annotations(annotations),
                    )
                    .footer(Level::Info.title(&footer_label)),
            )?;
        }

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
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

    fn enter_footnote_definition(
        &mut self,
        ast: &Ast,
        defn: &NodeFootnoteDefinition,
    ) -> Result<Next, Self::Error> {
        self.check(ast, &defn.name)
    }

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        if self.link_depth > 0 {
            if let Some(m) = self.re.find(txt) {
                self.linked.insert(m.as_str().to_owned());
            }
            Ok(Next::TraverseChildren)
        } else {
            self.check(ast, txt)
        }
    }

    fn enter_link(&mut self, _: &Ast, _: &NodeLink) -> Result<Next, Self::Error> {
        self.link_depth += 1;
        Ok(Next::TraverseChildren)
    }

    fn depart_link(&mut self, _: &Ast, _: &NodeLink) -> Result<(), Self::Error> {
        self.link_depth = self.link_depth.checked_sub(1).unwrap();
        Ok(())
    }

    fn enter_image(&mut self, _: &Ast, _: &NodeLink) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_footnote_reference(
        &mut self,
        ast: &Ast,
        refn: &NodeFootnoteReference,
    ) -> Result<Next, Self::Error> {
        self.check(ast, &refn.name)
    }
}
