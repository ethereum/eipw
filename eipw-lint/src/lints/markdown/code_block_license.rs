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
use std::sync::OnceLock;

fn spdx_regex() -> &'static TextRegex {
    static RE: OnceLock<TextRegex> = OnceLock::new();
    RE.get_or_init(|| {
        TextRegex::new(
            r"(?m)^\s*(?://|#|/\*+|\*)\s*SPDX-License-Identifier\s*:\s*(.+?)\s*(?:\*/)?\s*$",
        )
        .expect("hard-coded SPDX regex should be valid")
    })
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct CodeBlockLicense<S> {
    pub language: S,
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
            language: self.language.as_ref(),
            license: self.license.as_ref(),
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    slug: &'c str,
    language: &'c str,
    license: &'c str,
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_code_block(&mut self, ast: &Ast, node: &NodeCodeBlock) -> Result<Next, Self::Error> {
        let info = node.info.split_whitespace().next().unwrap_or("");
        if info != self.language {
            return Ok(Next::SkipChildren);
        }

        let captures = match spdx_regex().captures(&node.literal) {
            Some(c) => c,
            None => return Ok(Next::SkipChildren),
        };

        let actual = captures.get(1).unwrap().as_str();
        if actual == self.license {
            return Ok(Next::SkipChildren);
        }

        let label = format!(
            "code block of type `{}` must use license `{}`, not `{}`",
            self.language, self.license, actual,
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
