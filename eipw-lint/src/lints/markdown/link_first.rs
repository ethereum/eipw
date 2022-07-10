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

use std::collections::HashSet;

#[derive(Debug)]
pub struct LinkFirst<'n>(pub &'n str);

impl<'n> Lint for LinkFirst<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let re = BytesRegex::new(self.0).map_err(Error::custom)?;

        let mut visitor = Visitor {
            ctx,
            re,
            pattern: self.0,
            slug,
            linked: Default::default(),
            link_depth: 0,
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    re: BytesRegex,
    pattern: &'c str,
    slug: &'c str,
    linked: HashSet<Vec<u8>>,
    link_depth: usize,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast, buf: &[u8]) -> Result<Next, Error> {
        for matched in self.re.find_iter(buf) {
            if self.linked.contains(matched.as_bytes()) {
                continue;
            }

            let footer_label = format!("the pattern in question: `{}`", self.pattern);

            // TODO: Actually annotate the matches.

            let source = self.ctx.source_for_text(ast.start_line, buf);
            self.ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(self.slug),
                    label: Some("the first match of the given pattern must be a link"),
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
        }

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
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
        if self.link_depth > 0 {
            if let Some(m) = self.re.find(txt) {
                self.linked.insert(m.as_bytes().to_vec());
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

    fn enter_image(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.check(ast, &link.title)
    }

    fn enter_footnote_reference(&mut self, ast: &Ast, refn: &[u8]) -> Result<Next, Self::Error> {
        self.check(ast, refn)
    }
}
