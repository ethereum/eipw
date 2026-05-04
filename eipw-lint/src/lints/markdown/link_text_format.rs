/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::{Ast, NodeCode, NodeCodeBlock, NodeHtmlBlock, NodeLink};

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct LinkTextFormat;

impl Lint for LinkTextFormat {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let eip_url_re = Regex::new(r"(?i)^https?://(?:eips|ercs)\.ethereum\.org/(?:EIPS|ERCS)/(?:eip|erc)-(\d+)").unwrap();
        let text_re = Regex::new(r"(?i)^(?:eip|erc)-(\d+)$").unwrap();

        let mut visitor = Visitor {
            ctx,
            slug,
            eip_url_re,
            text_re,
            current_eip_url: None,
        };

        ctx.body().traverse().visit(&mut visitor)?;

        Ok(())
    }
}

struct Visitor<'a, 'b, 'c> {
    ctx: &'c Context<'a, 'b>,
    slug: &'c str,
    eip_url_re: regex::Regex,
    text_re: regex::Regex,
    current_eip_url: Option<String>,
}

impl<'a, 'b, 'c> Visitor<'a, 'b, 'c> {
    fn check_text(&self, ast: &Ast, txt: &str) -> Result<Next, Error> {
        let url = match &self.current_eip_url {
            Some(u) => u,
            None => return Ok(Next::TraverseChildren),
        };

        let url_caps = match self.eip_url_re.captures(url) {
            Some(caps) => caps,
            None => return Ok(Next::TraverseChildren),
        };

        let url_path = url_caps.get(0).map(|m| m.as_str()).unwrap_or("");
        let url_prefix = if url_path.to_lowercase().contains("/ercs/") {
            "erc"
        } else {
            "eip"
        };

        let text_lower = txt.to_lowercase();
        let text_prefix = if text_lower.starts_with("erc-") {
            "erc"
        } else if text_lower.starts_with("eip-") {
            "eip"
        } else {
            let label = "link text for EIP references must be in the format `EIP-N` or `ERC-N`";
            self.ctx.report(
                self.ctx.annotation_level()
                    .title(label)
                    .id(self.slug)
                    .snippet(self.ctx.ast_snippet(ast, None, None)),
            )?;
            return Ok(Next::TraverseChildren);
        };

        if url_prefix != text_prefix {
            let label = "link text for EIP references must be in the format `EIP-N` or `ERC-N`";
            self.ctx.report(
                self.ctx.annotation_level()
                    .title(label)
                    .id(self.slug)
                    .snippet(self.ctx.ast_snippet(ast, None, None)),
            )?;
        }

        Ok(Next::TraverseChildren)
    }
}

impl<'a, 'b, 'c> tree::Visitor for Visitor<'a, 'b, 'c> {
    type Error = Error;

    fn enter_front_matter(&mut self, _: &Ast, _: &str) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code(&mut self, _: &Ast, _: &NodeCode) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_code_block(&mut self, _: &Ast, _: &NodeCodeBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_inline(&mut self, _: &Ast, _: &str) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_html_block(&mut self, _: &Ast, _: &NodeHtmlBlock) -> Result<Next, Self::Error> {
        Ok(Next::SkipChildren)
    }

    fn enter_link(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        if self.eip_url_re.is_match(&link.url) {
            self.current_eip_url = Some(link.url.clone());
        }
        Ok(Next::TraverseChildren)
    }

    fn depart_link(&mut self, _: &Ast, _: &NodeLink) -> Result<(), Self::Error> {
        self.current_eip_url = None;
        Ok(())
    }

    fn enter_image(&mut self, ast: &Ast, link: &NodeLink) -> Result<Next, Self::Error> {
        if self.eip_url_re.is_match(&link.url) {
            self.current_eip_url = Some(link.url.clone());
        }
        Ok(Next::TraverseChildren)
    }

    fn depart_image(&mut self, _: &Ast, _: &NodeLink) -> Result<(), Self::Error> {
        self.current_eip_url = None;
        Ok(())
    }

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        self.check_text(ast, txt)
    }
}