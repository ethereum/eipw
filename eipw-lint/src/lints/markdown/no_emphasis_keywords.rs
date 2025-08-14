/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::Ast;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use regex::Regex;
use serde::{Deserialize, Serialize};

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct NoEmphasisKeywords;

impl Lint for NoEmphasisKeywords {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let mut visitor = Visitor {
            ctx,
            slug,
            in_emphasis: false,
            emphasis_type: EmphasisType::None,
            uppercase_regex: Regex::new(r"\b[A-Z]{2,}\b").map_err(Error::custom)?,
        };
        ctx.body().traverse().visit(&mut visitor)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum EmphasisType {
    None,
    Bold,
    Italic,
}

impl EmphasisType {
    fn description(&self) -> &'static str {
        match self {
            EmphasisType::None => "",
            EmphasisType::Bold => "bold",
            EmphasisType::Italic => "italic",
        }
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    slug: &'c str,
    in_emphasis: bool,
    emphasis_type: EmphasisType,
    uppercase_regex: Regex,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check_text(&mut self, ast: &Ast, text: &str) -> Result<Next, Error> {
        if !self.in_emphasis {
            return Ok(Next::TraverseChildren);
        }

        // Check if the text contains any uppercase keywords (2+ consecutive uppercase letters)
        if let Some(m) = self.uppercase_regex.find(text) {
            let keyword = m.as_str();
            let emphasis_desc = self.emphasis_type.description();
            let message = format!(
                "uppercase keywords should not be formatted with {} emphasis",
                emphasis_desc
            );
            
            self.ctx.report(
                self.ctx
                    .annotation_level()
                    .title(&message)
                    .id(self.slug)
                    .snippet(self.ctx.ast_snippet(ast, None, None))
                    .footer(Level::Info.title(&format!(
                        "uppercase keyword `{}` found in {} text", 
                        keyword, 
                        emphasis_desc
                    ))),
            )?;
        }

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_emph(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        self.in_emphasis = true;
        self.emphasis_type = EmphasisType::Italic;
        Ok(Next::TraverseChildren)
    }

    fn depart_emph(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        self.in_emphasis = false;
        self.emphasis_type = EmphasisType::None;
        Ok(())
    }

    fn enter_strong(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        self.in_emphasis = true;
        self.emphasis_type = EmphasisType::Bold;
        Ok(Next::TraverseChildren)
    }

    fn depart_strong(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        self.in_emphasis = false;
        self.emphasis_type = EmphasisType::None;
        Ok(())
    }

    fn enter_text(&mut self, ast: &Ast, text: &str) -> Result<Next, Self::Error> {
        self.check_text(ast, text)
    }
}
