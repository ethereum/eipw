/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use comrak::nodes::{Ast, NodeCodeBlock};
use jsonschema::output::BasicOutput;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use jsonschema::{CompilationOptions, JSONSchema};

use snafu::{FromString as _, Whatever};

#[derive(Debug)]
pub struct JsonSchema<'n> {
    pub language: &'n str,
    pub additional_schemas: &'n [(&'n str, &'n str)],
    pub schema: &'n str,
    pub help: &'n str,
}

impl<'n> Lint for JsonSchema<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let value: serde_json::Value = serde_json::from_str(self.schema).map_err(Error::custom)?;

        let mut options = CompilationOptions::default();

        options.with_draft(jsonschema::Draft::Draft7);

        for (url, json_text) in self.additional_schemas {
            let value: serde_json::Value =
                serde_json::from_str(json_text).map_err(Error::custom)?;
            options.with_document(url.to_string(), value);
        }

        let schema = options
            .compile(&value)
            .map_err(|e| Whatever::without_source(e.to_string()))
            .map_err(Error::custom)?;

        let mut visitor = Visitor {
            ctx,
            schema,
            slug,
            language: self.language,
            help: self.help,
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
    schema: JSONSchema,
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
                let source = self
                    .ctx
                    .source_for_text(ast.sourcepos.start.line, &node.literal);
                let slice_label = e.to_string();
                self.ctx.report(Snippet {
                    title: Some(Annotation {
                        annotation_type: self.ctx.annotation_type(),
                        id: Some(self.slug),
                        label: Some(&label),
                    }),
                    slices: vec![Slice {
                        fold: false,
                        line_start: ast.sourcepos.start.line,
                        origin: self.ctx.origin(),
                        source: &source,
                        annotations: vec![SourceAnnotation {
                            // TODO: The serde_json error actually has line/column
                            //       information. Use it.
                            annotation_type: self.ctx.annotation_type(),
                            label: &slice_label,
                            range: (0, source.len()),
                        }],
                    }],
                    ..Default::default()
                })?;
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
        let source = self
            .ctx
            .source_for_text(ast.sourcepos.start.line, &node.literal);
        let annotations = labels
            .iter()
            .map(|l| SourceAnnotation {
                annotation_type: self.ctx.annotation_type(),
                label: l,
                range: (0, source.len()),
            })
            .collect();

        let label = format!(
            "code block of type `{}` does not conform to required schema",
            info
        );
        self.ctx.report(Snippet {
            title: Some(Annotation {
                annotation_type: self.ctx.annotation_type(),
                id: Some(self.slug),
                label: Some(&label),
            }),
            slices: vec![Slice {
                fold: false,
                line_start: ast.sourcepos.start.line,
                origin: self.ctx.origin(),
                source: &source,
                annotations,
            }],
            footer: vec![Annotation {
                annotation_type: AnnotationType::Help,
                label: Some(self.help),
                id: None,
            }],
            ..Default::default()
        })?;

        Ok(Next::SkipChildren)
    }
}
