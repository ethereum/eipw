/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet};

use comrak::nodes::{Ast, AstNode, NodeCode, NodeCodeBlock, NodeHtmlBlock};

use crate::lints::{Context, Error, FetchContext, Lint};
use crate::tree::{self, Next, TraverseExt};

use regex::Regex;

use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProposalRef;

impl ProposalRef {
    fn find_refs<'a>(node: &'a AstNode<'a>) -> Result<Vec<(usize, PathBuf, String)>, Error> {
        let mut visitor = Visitor::default();
        node.traverse().visit(&mut visitor)?;
        Ok(visitor.refs)
    }
}

impl Lint for ProposalRef {
    fn find_resources<'a>(&self, ctx: &FetchContext<'a>) -> Result<(), Error> {
        Self::find_refs(ctx.body())?
            .into_iter()
            .map(|x| x.1)
            .collect::<HashSet<_>>()
            .into_iter()
            .for_each(|p| ctx.fetch(p));

        Ok(())
    }

    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        for (start_line, url, text) in Self::find_refs(ctx.body())? {
            let eip = match ctx.eip(&url) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", url.display(), e);
                    ctx.report(Snippet {
                        title: Some(Annotation {
                            id: Some(slug),
                            label: Some(&label),
                            annotation_type: ctx.annotation_type(),
                        }),
                        slices: vec![Slice {
                            fold: false,
                            line_start: start_line,
                            origin: ctx.origin(),
                            source: ctx.line(start_line),
                            annotations: vec![],
                        }],
                        ..Default::default()
                    })?;
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

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: start_line,
                    origin: ctx.origin(),
                    source: ctx.line(start_line),
                    annotations: vec![],
                }],
                ..Default::default()
            })?;
        }

        Ok(())
    }
}

struct Visitor {
    re: Regex,
    refs: Vec<(usize, PathBuf, String)>,
}

impl Default for Visitor {
    fn default() -> Self {
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

            let filename = format!("eip-{}.md", number_txt);

            self.refs
                .push((ast.sourcepos.start.line, filename.into(), whole.into()));
        }

        Ok(Next::TraverseChildren)
    }
}
