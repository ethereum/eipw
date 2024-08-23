/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeCode};

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use ::regex::Regex;

use serde::{Deserialize, Serialize};

// use std::collections::HashSet;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NoBackticks<S>(pub S);

impl<S> Lint for NoBackticks<S>
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
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check(&mut self, ast: &Ast, text: &str) -> Result<Next, Error> {
        if !self.re.is_match(text) {
            return Ok(Next::TraverseChildren);
        }

        let footer_label = format!("the pattern in question: `{}`", self.pattern);
        let source = self.ctx.source_for_text(ast.sourcepos.start.line, text);
        self.ctx.report(Snippet {
            opt: Default::default(),
            title: Some(Annotation {
                annotation_type: self.ctx.annotation_type(),
                id: Some(self.slug),
                label: Some("EIP references should not be in backticks"),
            }),
            slices: vec![Slice {
                fold: false,
                line_start: ast.sourcepos.start.line,
                origin: self.ctx.origin(),
                source: &source,
                annotations: vec![],
            }],
            footer: vec![Annotation {
                annotation_type: AnnotationType::Info,
                id: None,
                label: Some(&footer_label),
            }],
        })?;

        Ok(Next::SkipChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_code(&mut self, ast: &Ast, code: &NodeCode) -> Result<Next, Self::Error> {
        self.check(ast, &code.literal)
    }
}
