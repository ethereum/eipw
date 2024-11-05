/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::{Ast, AstNode, NodeCode, NodeCodeBlock, NodeHtmlBlock};
use eipw_snippets::Snippet;

use crate::lints::{Context, Error, FetchContext, Lint};
use crate::tree::{self, Next, TraverseExt};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProposalRef;

impl ProposalRef {
    fn find_refs<'a>(&self, node: &'a AstNode<'a>) -> Result<Vec<(Ast, u32, String)>, Error> {
        let mut visitor = Visitor::new();
        node.traverse().visit(&mut visitor)?;
        Ok(visitor.refs)
    }
}

impl Lint for ProposalRef {
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), Error> {
        self.find_refs(ctx.body())?
            .into_iter()
            .map(|x| x.1)
            .collect::<HashSet<_>>()
            .into_iter()
            .for_each(|p| ctx.fetch_proposal(p));

        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        for (ast, number, text) in self.find_refs(ctx.body())? {
            let eip = match ctx.proposal(number) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read proposal `{}`: {}", text, e);
                    ctx.report(
                        ctx.annotation_level()
                            .title(&label)
                            .id(slug)
                            .snippet(ctx.ast_snippet(&ast, None, None)),
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

            let source = ctx.ast_lines(&ast);
            let annotations = source.match_indices(&text).map(|(start, _)| {
                let end = start + text.len();
                ctx.annotation_level().span(start..end)
            });

            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(source)
                        .fold(true)
                        .line_start(ast.sourcepos.start.line)
                        .annotations(annotations),
                ),
            )?;
        }

        Ok(())
    }
}

struct Visitor {
    re: Regex,
    refs: Vec<(Ast, u32, String)>,
}

impl Visitor {
    fn new() -> Self {
        Self {
            // NB: This regex is used to calculate a path, so be careful of directory traversal.
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

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        for found in self.re.captures_iter(txt) {
            let whole = found.get(0).unwrap().as_str();
            let number_txt = found.get(1).unwrap().as_str();
            let number = number_txt
                .parse()
                .expect("bad numeric regex for ProposalRef");

            self.refs.push((ast.clone(), number, whole.into()));
        }

        Ok(Next::TraverseChildren)
    }
}
