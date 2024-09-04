/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};
 use comrak::nodes::Ast;
 use crate::lints::{Context, Error, Lint};
 use crate::tree::{self, Next, TraverseExt};
 use regex::Regex;
 use serde::{Deserialize, Serialize};
 use std::fmt::{Debug, Display};
 
 #[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct PreventUrlsNoBackticks<S>(pub S);
 
 impl<S> Lint for PreventUrlsNoBackticks<S> 
 where
     S: Debug + Display + AsRef<str>,
 {
     fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
         // Regex to match URLs not from allowed domains and containing backticks
         let re = Regex::new(&format!(
             r"https?://(?!{})(?:[^`\s]+[^`])", 
             self.0.as_ref().replace(".", r"\.")
         )).unwrap();
 
         let mut visitor = Visitor {
             ctx,
             re,
             slug,
         };
 
         ctx.body().traverse().visit(&mut visitor)?;
         Ok(())
     }
 }
 
 pub struct Visitor<'a, 'b, 'c> {
     ctx: &'c Context<'a, 'b>,
     re: Regex,
     slug: &'c str,
 }
 
 impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
     fn check(&mut self, ast: &Ast, text: &str) -> Result<Next, Error> {
         // Check if the text matches the regex pattern
         if !self.re.is_match(text) {
             return Ok(Next::TraverseChildren);
         }
 
         // Report the issue
         let source = self.ctx.source_for_text(ast.sourcepos.start.line, text);
         let offending_url = self.re.find(text).unwrap().as_str();
         let suggestion_label = format!("Avoid using backticks in URLs: `{}`", offending_url);
 
         self.ctx.report(Snippet {
             opt: Default::default(),
             title: Some(Annotation {
                 annotation_type: self.ctx.annotation_type(),
                 id: Some(self.slug),
                 label: Some("URL containing backticks or not from allowed domain found"),
             }),
             slices: vec![Slice {
                 fold: false,
                 line_start: ast.sourcepos.start.line,
                 origin: self.ctx.origin(),
                 source: &source,
                 annotations: vec![],
             }],
             footer: vec![Annotation {
                 annotation_type: AnnotationType::Help,
                 id: None,
                 label: Some(&suggestion_label),
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
 
     fn enter_text(&mut self, ast: &Ast, text: &str) -> Result<Next, Self::Error> {
         self.check(ast, text)
     }
 } 