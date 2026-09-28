/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use comrak::nodes::{Ast, NodeCodeBlock};

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::SnippetExt;

use ::regex::Regex as TextRegex;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

lazy_static::lazy_static! {
    static ref SPDX: TextRegex =
        TextRegex::new(r"(?m)SPDX-License-Identifier\s*:\s*(.+?)\s*(?:\*/)?\s*$").unwrap();
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct CodeBlockLicense<S> {
    pub license: S,
}

impl<S> Lint for CodeBlockLicense<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let mut visitor = Visitor {
            ctx,
            slug,
            license: self.license.as_ref(),
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    slug: &'c str,
    license: &'c str,
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_code_block(&mut self, ast: &Ast, node: &NodeCodeBlock) -> Result<Next, Self::Error> {
        let captures = match SPDX.captures(&node.literal) {
            Some(c) => c,
            None => return Ok(Next::SkipChildren),
        };

        let actual = captures.get(1).unwrap().as_str();
        if actual == self.license {
            return Ok(Next::SkipChildren);
        }

        let label = format!(
            "code block must use license `{}`, not `{}`",
            self.license, actual,
        );
        let source = self.ctx.ast_lines(ast);
        self.ctx.report(
            self.ctx
                .annotation_level()
                .title(&label)
                .id(self.slug)
                .snippet(
                    Snippet::source(source)
                        .fold(true)
                        .line_start(ast.sourcepos.start.line)
                        .origin_opt(self.ctx.origin())
                        .annotation(self.ctx.annotation_level().span(0..source.len())),
                ),
        )?;

        Ok(Next::SkipChildren)
    }
}
