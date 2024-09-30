/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


 use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

 use comrak::nodes::Ast;
 
 use crate::lints::{Context, Error, Lint};
 use crate::tree::{self, Next, TraverseExt};
 
 use regex::{Regex, RegexSet};
 
 use scraper::node::Node as HtmlNode;
 use scraper::Html;
 
 use serde::{Deserialize, Serialize};
 
 use snafu::Snafu;
 
 use std::fmt::{Debug, Display};


 #[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct PreventUrlsNoBackticks<S> {
    pub allowed_domains: Vec<S>,
 }
 
 impl<S> Lint for PreventUrlsNoBackticks<S> 
 where
     S: Debug + Display + AsRef<str>,
 {
     fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        
         // Regex to match URLs not from allowed domains and containing backtick

        let re = Regex::new(r"https?://(?:example\.com|example\.net|example\.org|example|invalid|test)(?:/[^`\s]*)?").unwrap();

        let allowed_domains = RegexSet::new(
            self.allowed_domains
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
        ).map_err(Error::custom)?;

        let mut visitor = Visitor {
            ctx,
            re,
            allowed_domains,
        };
 
        ctx.body().traverse().visit(&mut visitor)?;
         Ok(())
     }
 }
 
 pub struct Visitor<'a, 'b, 'c> {
     ctx: &'c Context<'a, 'b>,
     re: Regex,
     allowed_domains: RegexSet,
 }
 
 impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
     fn check(&mut self, ast: &Ast, text: &str, is_link: bool, in_backticks: bool) -> Result<Next, Error> {
         // Check if the text matches the regex pattern       
         for mat in self.re.find_iter(text) {
            let url = mat.as_str();
            let domain = url.split('/').nth(2).unwrap_or("");

             // If the URL is in backticks, report it
             if in_backticks {
                let source = self.ctx.source_for_text(ast.sourcepos.start.line, text);
                let message = format!("URLs are not allowed in backticks: `{}`", url);

                self.ctx.report(Snippet {
                    opt: Default::default(),
                    title: Some(Annotation {
                        annotation_type: self.ctx.annotation_type(),
                        id: Some(self.slug),
                        label: Some("not from allowed domain found"),
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
                continue;
             }

             // If the URL is in an allowed domain, skip it
            if self.allowed_domains.is_match(domain) {
                continue;
            }
            // Skip if the URL is part of a markdown link or HTML link
            if is_link {
                continue;
            }

            // Report other disallowed URLs
            let source = self.ctx.source_for_text(ast.sourcepos.start.line, text);
            let suggestion_label = format!("This URL must be hyperlinked or from an allowed domain: `{}`", url);

            self.ctx.report(Snippet {
                opt: Default::default(),
                title: Some(Annotation {
                    annotation_type: self.ctx.annotation_type(),
                    id: Some(self.slug),
                    label: Some("Disallowed URL in plain text or code fence"),
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
        }
         
        
 
    
         
 
         Ok(Next::SkipChildren)
     }
 }
 
 impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
     type Error = Error;

     fn enter_link(&mut self, _ast: &Ast, _link: &str) -> Result<Next, Self::Error> {
            Ok(Next::TraverseChildren)
     }
 
     fn enter_code(&mut self, ast: &Ast, code: &comrak::nodes::NodeCode) -> Result<Next, Self::Error> {
         self.check(ast, &code.literal, false, false)
     }
 
     fn enter_text(&mut self, ast: &Ast, text: &str) -> Result<Next, Self::Error> {
         self.check(ast, text, false, false)
     }

     fn enter_html_inline(&mut self, ast: &Ast, html: &str) -> Result<Next, Self::Error> {
        // For inline HTML links like <https://link>
        self.check(ast, html, true, false)
    }

    fn enter_code_block(&mut self, ast: &Ast, block: &comrak::nodes::NodeCodeBlock) -> Result<Next, Self::Error> {
        let code_text = String::from_utf8_lossy(&block.literal);
        self.check(ast, &code_text, false, false)
    }


 } 