/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

const COPYRIGHT_HEADING: &str = "## Copyright";
const COPYRIGHT_WAIVER: &str = "Copyright and related rights waived via [CC0](../LICENSE.md).";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct Copyright;

impl Lint for Copyright {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let copyright_heading = ctx
            .body()
            .descendants()
            .find(|node| match &*node.data.borrow() {
                Ast {
                    value: NodeValue::Heading(NodeHeading { level: 2, .. }),
                    ..
                } => {
                    let text = node
                        .descendants()
                        .skip(1)
                        .filter_map(|child| match &*child.data.borrow() {
                            Ast {
                                value: NodeValue::Text(text),
                                ..
                            } => Some(text.to_owned()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    text == "Copyright"
                }
                _ => false,
            });

        let Some(copyright_heading) = copyright_heading else {
            return Ok(());
        };

        let non_blank_lines: Vec<_> = ctx
            .body_source()
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        if !non_blank_lines.contains(&COPYRIGHT_WAIVER) {
            return Ok(());
        }

        if non_blank_lines.ends_with(&[COPYRIGHT_HEADING, COPYRIGHT_WAIVER]) {
            return Ok(());
        }

        ctx.report(
            ctx.annotation_level()
                .title("copyright waiver must appear at the end of the file")
                .id(slug)
                .snippet(ctx.ast_snippet(
                    &copyright_heading.data.borrow(),
                    None,
                    "move this waiver to the end",
                ))
                .footer(Level::Help.title(
                    "end the file with `## Copyright` followed by `Copyright and related rights waived via [CC0](../LICENSE.md).`",
                )),
        )?;

        Ok(())
    }
}
