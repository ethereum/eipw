/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::NodeValue;

use crate::lints::{Context, Error, Lint};

use scraper::node::Node as HtmlNode;
use scraper::Html;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlComments<S> {
    pub name: S,
    pub warn_for: Vec<S>,
}

impl<S> Lint for HtmlComments<S>
where
    S: Display + Debug + AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(s) => s.value().trim(),
        };

        let warn = self.warn_for.iter().any(|e| e == &field);

        // Downgrade diagnostic level if header's value is in `warn_for`.
        let annotation_type = if warn && ctx.annotation_level() == Level::Error {
            Level::Warning
        } else {
            ctx.annotation_level()
        };

        let mut slices = vec![];

        for node in ctx.body().descendants() {
            let data = node.data.borrow();
            let fragment = match data.value {
                NodeValue::HtmlBlock(ref b) => Html::parse_fragment(&b.literal),
                _ => continue,
            };

            for node in fragment.tree.nodes() {
                if !matches!(node.value(), HtmlNode::Comment(_)) {
                    continue;
                }

                slices.push(ctx.ast_snippet(&data, annotation_type, None));
            }
        }

        if !slices.is_empty() {
            let label = match warn {
                true => {
                    let joined = self
                        .warn_for
                        .iter()
                        .map(AsRef::as_ref)
                        .collect::<Vec<_>>()
                        .join("`, `");
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

            ctx.report(annotation_type.title(&label).id(slug).snippets(slices))?;
        }

        Ok(())
    }
}
