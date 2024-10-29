/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::Ast;

use crate::lints::{Context, Error, Lint};
use crate::tree::{self, Next, TraverseExt};

use regex::{Regex, RegexSet};

use scraper::node::Node as HtmlNode;
use scraper::Html;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct RelativeLinks<S> {
    pub exceptions: Vec<S>,
}

const RE_URL: &str =
    r"^(https?:)?//(?:eips|ercs)\.ethereum\.org/(?:EIPS|ERCS)/(?:eip|erc)-(\d+)|(assets/.+)$";

impl<S> Lint for RelativeLinks<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let re = Regex::new("(^/)|(://)").unwrap();
        let eip_re = Regex::new(RE_URL).unwrap();

        let exceptions = RegexSet::new(&self.exceptions).map_err(Error::custom)?;

        let mut visitor = Visitor::default();
        ctx.body().traverse().visit(&mut visitor)?;

        let links = visitor.links.into_iter();

        for of_interest in links {
            let (address, ast) = match of_interest {
                OfInterest::Link { address, ast } => (address, ast),
                OfInterest::Unsupported { what, ast } => {
                    ctx.report(
                        ctx.annotation_level()
                            .title(&format!("unsupported HTML {what}"))
                            .id(slug)
                            .snippet(ctx.ast_snippet(&ast, None, what)),
                    )?;
                    continue;
                }
            };

            if !re.is_match(&address) {
                continue;
            }

            if exceptions.is_match(&address) {
                continue;
            }

            let (suggestion, extra_help) = if let Some(caps) = eip_re.captures(&address) {
                if let Some(id_number) = caps.get(2) {
                    let suggestion = format!("./eip-{}.md", id_number.as_str());
                    (suggestion, true)
                } else if let Some(asset_path) = caps.get(3) {
                    let suggestion = format!("../{}", asset_path.as_str());
                    (suggestion, true)
                } else {
                    (address, false)
                }
            } else if address.contains("//creativecommons.org/publicdomain/zero/1.0/") {
                ("../LICENSE.md".to_string(), true)
            } else {
                (address, false)
            };

            let mut footer = vec![];

            let suggestion_label = format!("use `{}` instead", suggestion);
            if extra_help {
                footer.push(Level::Help.title(&suggestion_label));
            }

            ctx.report(
                ctx.annotation_level()
                    .title("non-relative link or image")
                    .id(slug)
                    .footers(footer)
                    .snippet(ctx.ast_snippet(&ast, None, "used here")),
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum OfInterest {
    Unsupported { what: &'static str, ast: Ast },
    Link { address: String, ast: Ast },
}

#[derive(Debug, Default)]
struct Visitor {
    links: Vec<OfInterest>,
}

impl Visitor {
    fn push(&mut self, ast: &Ast, address: &str) -> Result<Next, <Self as tree::Visitor>::Error> {
        self.links.push(OfInterest::Link {
            address: address.to_owned(),
            ast: ast.clone(),
        });

        Ok(Next::TraverseChildren)
    }

    fn html(&mut self, ast: &Ast, html: &str) -> Result<Next, <Self as tree::Visitor>::Error> {
        let fragment = Html::parse_fragment(html);

        for node in fragment.tree.nodes() {
            let elem = match node.value() {
                HtmlNode::Element(e) => e,
                _ => continue,
            };

            if elem.name().eq_ignore_ascii_case("style")
                || elem.id().unwrap_or_default().eq_ignore_ascii_case("style")
            {
                self.links.push(OfInterest::Unsupported {
                    ast: ast.clone(),
                    what: "style tag",
                });
                continue;
            }

            for attr in elem.attrs() {
                if attr.0.eq_ignore_ascii_case("style") {
                    self.links.push(OfInterest::Unsupported {
                        ast: ast.clone(),
                        what: "style attribute",
                    });
                    continue;
                }

                self.push(ast, attr.1)?;
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

    fn enter_html_inline(&mut self, ast: &Ast, html: &str) -> Result<Next, Self::Error> {
        self.html(ast, html)
    }
}
