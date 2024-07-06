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
pub struct LinkOther<S>(pub S);

impl<S> Lint for LinkOther<S>
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
            current_link: Link {
                url: String::new(),
                text: String::new(),
            },
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
    fn extract_capture(&self, text: &str, re: &Regex, index: usize) -> Result<String, Error> {
        if let Some(captures) = re.captures(text) {
            Ok(captures
                .get(index)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default())
        } else {
            Ok(String::new())
        }
    }

    fn check(&self, ast: &Ast) -> Result<Next, Error> {
        let text_eip_full = self.extract_capture(&self.current_link.text, &self.re, 1)?;
        let text_eip_number = self.extract_capture(&self.current_link.text, &self.re, 2)?;

        let dynamic_pattern = format!(r"(?i)\beip-{}\b", regex::escape(&text_eip_number));
        let url_re = Regex::new(&dynamic_pattern).map_err(Error::custom)?;

        if url_re.is_match(&self.current_link.url) {
            return Ok(Next::TraverseChildren);
        }

        let expected = format!(
            "[{}](./{}.md)",
            text_eip_full.to_uppercase(),
            text_eip_full.to_lowercase()
        );
        let footer_label = format!(
            "the link destination should target {}, for example `{}`",
            text_eip_full.to_uppercase(),
            expected
        );

        let source = self
            .ctx
            .source_for_text(ast.sourcepos.start.line, &self.current_link.text);
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
                annotation_type: AnnotationType::Help,
                label: Some(&footer_label),
            }],
            opt: Default::default(),
        })?;

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_link(&mut self, _: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        self.current_link = Link {
            url: link.url.to_owned(),
            text: String::new(),
        };
        self.link_depth += 1;
        Ok(Next::TraverseChildren)
    }

    fn depart_link(&mut self, _: &Ast, _: &NodeLink) -> Result<(), Self::Error> {
        if self.link_depth > 0 {
            self.link_depth -= 1;
        }
        Ok(())
    }

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        if self.link_depth > 0 && self.re.is_match(&txt) {
            self.current_link.text = txt.to_owned();
            self.check(ast)?;
        }
        Ok(Next::TraverseChildren)
    }
}
