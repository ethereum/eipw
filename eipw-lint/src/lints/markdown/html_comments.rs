/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::NodeValue;

use crate::lints::{Context, Error, Lint};

use scraper::node::Node as HtmlNode;
use scraper::Html;

#[derive(Debug)]
pub struct HtmlComments<'n> {
    pub name: &'n str,
    pub warn_for: &'n [&'n str],
}

impl<'n> Lint for HtmlComments<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name) {
            None => return Ok(()),
            Some(s) => s.value().trim(),
        };

        let warn = self.warn_for.contains(&field);

        // Downgrade diagnostic level if header's value is in `warn_for`.
        let annotation_type = if warn && ctx.annotation_type() == AnnotationType::Error {
            AnnotationType::Warning
        } else {
            ctx.annotation_type()
        };

        let mut slices = vec![];

        for node in ctx.body().descendants() {
            let data = node.data.borrow();
            let fragment = match data.value {
                NodeValue::HtmlBlock(ref b) => {
                    let html = std::str::from_utf8(&b.literal)?;
                    Html::parse_fragment(html)
                }
                _ => continue,
            };

            for node in fragment.tree.nodes() {
                if !matches!(node.value(), HtmlNode::Comment(_)) {
                    continue;
                }

                slices.push(Slice {
                    line_start: usize::try_from(data.start_line).unwrap(),
                    fold: false,
                    origin: ctx.origin(),
                    source: ctx.line(data.start_line),
                    annotations: vec![],
                });
            }
        }

        if !slices.is_empty() {
            let label = match warn {
                true => {
                    let joined = self.warn_for.join("`, `");
                    format!(
                        "HTML comments are only allowed while `{}` is one of: `{joined}`",
                        self.name,
                    )
                }
                false => format!(
                    "HTML comments are not allowed when `{}` is `{field}`",
                    self.name,
                ),
            };

            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type,
                    label: Some(&label),
                }),
                footer: vec![],
                slices,
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
