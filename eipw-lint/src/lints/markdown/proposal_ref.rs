/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use comrak::nodes::{Ast, AstNode, NodeCode, NodeCodeBlock, NodeHtmlBlock};

use crate::lints::{Context, Error, FetchContext, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::SnippetExt;

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProposalRef<S> {
    pub prefix: S,
    pub suffix: S,
}

impl<S> ProposalRef<S>
where
    S: AsRef<str>,
{
    fn find_refs<'a>(&self, node: &'a AstNode<'a>) -> Result<Vec<(usize, PathBuf, String)>, Error> {
        let mut visitor = Visitor::new(self.prefix.as_ref(), self.suffix.as_ref());
        node.traverse().visit(&mut visitor)?;
        Ok(visitor.refs)
    }
}

impl<S> Lint for ProposalRef<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), Error> {
        self.find_refs(ctx.body())?
            .into_iter()
            .map(|x| x.1)
            .collect::<HashSet<_>>()
            .into_iter()
            .for_each(|p| ctx.fetch(p));

        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        for (start_line, url, text) in self.find_refs(ctx.body())? {
            let eip = match ctx.eip(&url) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", url.display(), e);
                    ctx.report(
                        ctx.annotation_level().title(&label).id(slug).snippet(
                            Snippet::source(ctx.line(start_line))
                                .fold(false)
                                .origin_opt(ctx.origin())
                                .line_start(start_line),
                        ),
                    )?;
                    continue;
                }
            };

            let category = eip.preamble().by_name("category").map(|f| f.value().trim());

            let prefix = match category {
                Some("ERC") => "ERC",
                _ => "EIP",
            };

            if text.starts_with(prefix) {
                continue;
            }

            let category_msg = match category {
                Some(c) => format!("with a `category` of `{}`", c),
                None => "without a `category`".to_string(),
            };

            let label = format!(
                "references to proposals {} must use a prefix of `{}`",
                category_msg, prefix,
            );

            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(ctx.line(start_line))
                        .line_start(start_line)
                        .fold(false)
                        .origin_opt(ctx.origin()),
                ),
            )?;
        }

        Ok(())
    }
}

struct Visitor<'a> {
    re: Regex,
    refs: Vec<(usize, PathBuf, String)>,
    prefix: &'a str,
    suffix: &'a str,
}

impl<'a> Visitor<'a> {
    fn new(prefix: &'a str, suffix: &'a str) -> Self {
        Self {
            // NB: This regex is used to calculate a path, so be careful of directory traversal.
            re: Regex::new(r"(?i)\b(?:eip|erc)-([0-9]+)\b").unwrap(),
            refs: Default::default(),
            prefix,
            suffix,
        }
    }
}

impl<'a> tree::Visitor for Visitor<'a> {
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

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        for found in self.re.captures_iter(txt) {
            let whole = found.get(0).unwrap().as_str();
            let number_txt = found.get(1).unwrap().as_str();

            let filename = format!("{}{}{}", self.prefix, number_txt, self.suffix);

            self.refs
                .push((ast.sourcepos.start.line, filename.into(), whole.into()));
        }

        Ok(Next::TraverseChildren)
    }
}
