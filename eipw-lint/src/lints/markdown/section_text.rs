/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct SectionText<S> {
    /// Text of the heading introducing the section.
    pub section: S,

    /// Level (starting at 1) of the heading to match on.
    pub level: u8,

    /// The section must be the last content in the file, and its body must be
    /// exactly this text (ignoring surrounding whitespace).
    pub exactly: S,
}

impl<S> SectionText<S>
where
    S: AsRef<str>,
{
    /// The source line the heading is expected to appear as, e.g. `## Copyright`.
    fn heading_line(&self) -> String {
        format!(
            "{} {}",
            "#".repeat(usize::from(self.level)),
            self.section.as_ref()
        )
    }
}

impl<S> Lint for SectionText<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let heading = ctx.body().descendants().find(|node| {
            let ast = node.data.borrow();

            let is_match = matches!(
                &*ast,
                Ast {
                    value: NodeValue::Heading(NodeHeading { level, .. }),
                    ..
                } if *level == self.level
            );

            if !is_match {
                return false;
            }

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

            text == self.section.as_ref()
        });

        let Some(heading) = heading else {
            return Ok(());
        };

        let non_blank_lines: Vec<_> = ctx
            .body_source()
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        if non_blank_lines.ends_with(&[self.heading_line().as_str(), self.exactly.as_ref()]) {
            return Ok(());
        }

        ctx.report(
            ctx.annotation_level()
                .title(&format!(
                    "the `{}` section must be the last content in the file",
                    self.section
                ))
                .id(slug)
                .snippet(ctx.ast_snippet(
                    &heading.data.borrow(),
                    None,
                    "nothing may follow this section",
                ))
                .footer(Level::Help.title(&format!(
                    "end the file with `{}` followed immediately by `{}`, with no other content after it",
                    self.heading_line(),
                    self.exactly,
                ))),
        )?;

        Ok(())
    }
}
