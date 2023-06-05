/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file. You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::Ast;

use std::fmt::Write;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use regex::bytes::{Regex, RegexSet};
use std::str;
use std::string::String;

use scraper::node::Node as HtmlNode;
use scraper::Html;

use snafu::Snafu;

#[derive(Debug)]
pub struct RelativeLinks<'e> {
    pub exceptions: &'e [&'e str],
}

impl<'e> Lint for RelativeLinks<'e> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let re = Regex::new("(^/)|(://)").unwrap();
        let re_eip_num = Regex::new(r"eip-\d{1,4}").unwrap();

        let exceptions = RegexSet::new(self.exceptions).map_err(Error::custom)?;

        let mut visitor = Visitor::default();
        ctx.body().traverse().visit(&mut visitor)?;

        let links = visitor
            .links
            .into_iter()
            .filter(|l| re.is_match(&l.address) && !exceptions.is_match(&l.address));

        for Link {
            line_start,
            address,
            ..
        } in links
        {
            let mut footer_label = String::new();
            let mut footer = vec![];
            let line_with_address = str::from_utf8(&address).unwrap();
            let mut test_assets_string = String::new();
            if let Some(num) = re_eip_num.captures(line_with_address.as_bytes()) {
                let eip_num = str::from_utf8(&num[0]).unwrap();
                write!(test_assets_string, "assets/{0}/eth_sign.png", &eip_num).unwrap();
                if !(line_with_address.contains(&test_assets_string)) {
                    write!(footer_label, "use `./{0}.md` instead", &eip_num).unwrap();
                } else {
                    write!(footer_label, "use `../{0}` instead", &test_assets_string).unwrap();
                }
                footer.push(Annotation {
                    annotation_type: AnnotationType::Help,
                    id: None,
                    label: Some(&footer_label),
                });
            }

            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: ctx.annotation_type(),
                    label: Some("non-relative link or image"),
                }),
                footer,
                slices: vec![Slice {
                    line_start: usize::try_from(line_start).unwrap(),
                    fold: false,
                    origin: ctx.origin(),
                    source: ctx.line(line_start),
                    annotations: vec![],
                }],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
struct Unsupported;

#[derive(Debug)]
struct Link {
    address: Vec<u8>,
    line_start: u32,
}

#[derive(Debug, Default)]
struct Visitor {
    links: Vec<Link>,
}

impl Visitor {
    fn push(&mut self, ast: &Ast, address: &[u8]) -> Result<Next, <Self as tree::Visitor>::Error> {
        self.links.push(Link {
            address: address.to_owned(),
            line_start: ast.start_line,
        });

        Ok(Next::TraverseChildren)
    }

    fn html(&mut self, ast: &Ast, html: &[u8]) -> Result<Next, <Self as tree::Visitor>::Error> {
        let html = std::str::from_utf8(html)?;
        let fragment = Html::parse_fragment(html);

        for node in fragment.tree.nodes() {
            let elem = match node.value() {
                HtmlNode::Element(e) => e,
                _ => continue,
            };

            if elem.name().eq_ignore_ascii_case("style")
                || elem.id().unwrap_or_default().eq_ignore_ascii_case("style")
            {
                return Err(Error::custom(Unsupported));
            }

            for attr in elem.attrs() {
                if attr.0.eq_ignore_ascii_case("style") {
                    return Err(Error::custom(Unsupported));
                }

                self.push(ast, attr.1.as_bytes())?;
            }
        }

        Ok(Next::TraverseChildren)
    }
}

impl tree::Visitor for Visitor {
    type Error = Error;

    fn enter_image(
        &mut self,
        ast: &Ast,
        link: &comrak::nodes::NodeLink,
    ) -> Result<Next, Self::Error> {
        self.push(ast, &link.url)
    }

    fn enter_link(
        &mut self,
        ast: &Ast,
        link: &comrak::nodes::NodeLink,
    ) -> Result<Next, Self::Error> {
        self.push(ast, &link.url)
    }

    fn enter_html_block(
        &mut self,
        ast: &Ast,
        html_block: &comrak::nodes::NodeHtmlBlock,
    ) -> Result<Next, Self::Error> {
        self.html(ast, &html_block.literal)
    }

    fn enter_html_inline(&mut self, ast: &Ast, html: &[u8]) -> Result<Next, Self::Error> {
        self.html(ast, html)
    }
}
