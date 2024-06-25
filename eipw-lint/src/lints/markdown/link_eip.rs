/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeLink};
 
use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};
 
use regex::Regex;
 
use serde::{Deserialize, Serialize};
 
use std::fmt::{Debug, Display};
 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkEip<S>(pub S);
 
impl<S> Lint for LinkEip<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let pattern = self.0.as_ref();
        let re = Regex::new(pattern).map_err(Error::custom)?;

        let mut visitor = Visitor {
            ctx,
            re,
            slug,
            link_depth: 0,
            current_link: Link { url: String::new(), text: String::new() },
        };
        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}
#[derive(Debug)]
struct Link {
    url: String,
    text: String,
}

#[derive(Debug)]
struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    re: Regex,
    slug: &'c str,
    link_depth: usize,
    current_link: Link,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check(&self, ast: &Ast) -> Result<Next, Error> {  
        let pattern = r"(?i)\b(?:eip|erc)-(\d+)\b";
        let url_re = Regex::new(pattern).map_err(Error::custom)?;

        let url_eip_number = if let Some(captures) = url_re.captures(&self.current_link.url) {
            captures.get(1).map(|m| m.as_str())
        } else { None };

        if let Some(url_eip_number) = url_eip_number {
            let section_pattern = r"eip-([^.]*)\.md#.+$";
            let url_re = Regex::new(section_pattern).map_err(Error::custom)?;
            let dynamic_pattern = if url_re.is_match(&self.current_link.url) {
                format!(r"^(EIP|ERC)-{}\s*\S+", regex::escape(&url_eip_number))
            } else {
                format!(r"^(EIP|ERC)-{}$", regex::escape(&url_eip_number))
            };
            let text_re = Regex::new(&dynamic_pattern).map_err(Error::custom)?;
        
            if text_re.is_match(&self.current_link.text) {
                return Ok(Next::TraverseChildren);
            };
        
            let expected = if url_re.is_match(&self.current_link.url) {
                format!("[EIP|ERC-{}<section-description>]", url_eip_number)
            } else {
                format!("[EIP|ERC-{}]", url_eip_number)
            };
        
            let footer_label = format!("link text should match `{}`", expected);
        
            let source = self.ctx.source_for_text(ast.sourcepos.start.line, &self.current_link.text);
            self.ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: self.ctx.annotation_type(),
                    id: Some(self.slug),
                    label: Some("link text does not match link destination"),
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

    fn enter_link(&mut self, _: &Ast, link: &NodeLink,) -> Result<Next, Self::Error> {
        if self.re.is_match(&link.url) {        
            self.current_link = Link { url: link.url.to_owned(), text: String::new() }; 
            self.link_depth += 1;  
        }                          
        Ok(Next::TraverseChildren)
    }

    fn depart_link(&mut self, _: &Ast, _: &NodeLink) -> Result<(), Self::Error> {
        if self.link_depth > 0 {
            self.link_depth = self.link_depth.checked_sub(1).unwrap();
        }
        Ok(())
    }

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        if self.link_depth > 0 {
            self.current_link.text = txt.to_owned();
            self.check(ast)?;         
        }
        Ok(Next::TraverseChildren)    
    }
}