/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeCode, NodeCodeBlock, NodeHtmlBlock, NodeLink};

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

        let mut visitor = Visitor {
            ctx,
            re,
            pattern,
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
    re: Regex,
    pattern: &'c str,
    slug: &'c str,
    linked: HashSet<String>,
    link_depth: usize,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast, text: &str) -> Result<Next, Error> {
        for matched in self.re.find_iter(text) {
            if self.linked.contains(matched.as_str()) {
                continue;
            }

            let footer_label = format!("the pattern in question: `{}`", self.pattern);

            // TODO: Actually annotate the matches.

            let source = self.ctx.source_for_text(ast.sourcepos.start.line, text);
            self.ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: self.ctx.annotation_type(),
                    id: Some(self.slug),
                    label: Some("the first match of the given pattern must be a link"),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: ast.sourcepos.start.line,
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

    fn enter_image(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.check(ast, &link.title)
    }

    fn enter_footnote_reference(&mut self, ast: &Ast, refn: &str) -> Result<Next, Self::Error> {
        self.check(ast, refn)
    }
}
