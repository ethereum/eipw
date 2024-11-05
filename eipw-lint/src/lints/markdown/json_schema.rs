/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Snippet};

use comrak::nodes::{Ast, NodeCodeBlock};
use jsonschema::output::BasicOutput;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
use crate::SnippetExt;

use jsonschema::{Resource, ValidationOptions, Validator};

use serde::{Deserialize, Serialize};

use snafu::{FromString as _, Whatever};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonSchema<S> {
    pub language: S,
    pub additional_schemas: Vec<(S, S)>,
    pub schema: S,
    pub help: S,
}

impl<S> Lint for JsonSchema<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let value: serde_json::Value =
            serde_json::from_str(self.schema.as_ref()).map_err(Error::custom)?;

        let mut options = ValidationOptions::default();

        options.with_draft(jsonschema::Draft::Draft7);

        for (url, json_text) in &self.additional_schemas {
            let value: serde_json::Value =
                serde_json::from_str(json_text.as_ref()).map_err(Error::custom)?;
            let resource = Resource::from_contents(value).map_err(Error::custom)?;
            options.with_resource(url.to_string(), resource);
        }

        let schema = options
            .build(&value)
            .map_err(|e| Whatever::without_source(e.to_string()))
            .map_err(Error::custom)?;

        let mut visitor = Visitor {
            ctx,
            schema,
            slug,
            language: self.language.as_ref(),
            help: self.help.as_ref(),
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    language: &'c str,
    slug: &'c str,
    help: &'c str,
    schema: Validator,
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_code_block(&mut self, ast: &Ast, node: &NodeCodeBlock) -> Result<Next, Self::Error> {
        let info = &node.info;
        if info != self.language {
            return Ok(Next::SkipChildren);
        }

        let json_value: serde_json::Value = match serde_json::from_str(&node.literal) {
            Ok(v) => v,
            Err(e) => {
                let label = format!("code block of type `{}` does not contain valid JSON", info);
                let slice_label = e.to_string();
                self.ctx.report(
                    self.ctx
                        .annotation_level()
                        .title(&label)
                        .id(self.slug)
                        .snippet(
                            // TODO: The serde_json error actually has line/column
                            //       information. Use it.
                            self.ctx.ast_snippet(ast, None, slice_label.as_str()),
                        ),
                )?;
                return Ok(Next::SkipChildren);
            }
        };

        let errors = match self.schema.apply(&json_value).basic() {
            BasicOutput::Valid(_) => return Ok(Next::SkipChildren),
            BasicOutput::Invalid(e) => e,
        };

        let labels: Vec<_> = errors
            .into_iter()
            .map(|d| d.error_description().to_string())
            .collect();
        let source = self.ctx.ast_lines(ast);
        let annotations = labels
            .iter()
            .map(|l| self.ctx.annotation_level().span(0..source.len()).label(l));

        let label = format!(
            "code block of type `{}` does not conform to required schema",
            info
        );
        self.ctx.report(
            self.ctx
                .annotation_level()
                .title(&label)
                .id(self.slug)
                .snippet(
                    Snippet::source(source)
                        .fold(false)
                        .line_start(ast.sourcepos.start.line)
                        .origin_opt(self.ctx.origin())
                        .annotations(annotations),
                )
                .footer(Level::Help.title(self.help)),
        )?;

        Ok(Next::SkipChildren)
    }
}
