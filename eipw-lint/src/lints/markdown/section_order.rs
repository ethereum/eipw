/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt::{Debug, Display, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SectionOrder<S>(pub Vec<S>);

impl<S> SectionOrder<S>
where
    S: AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn find_preceding(&self, present: &[&str], needle: &str) -> Option<&str> {
        let needle_idx = match self.0.iter().position(|x| *x == needle) {
            None | Some(0) => return None,
            Some(i) => i,
        };

        for (idx, name) in self.0.iter().enumerate().rev() {
            let name = name.as_ref();
            if name == needle || idx >= needle_idx {
                continue;
            }

            if present.iter().any(|e| e == &name) {
                return Some(name);
            }
        }

        None
    }
}

impl<S> Lint for SectionOrder<S>
where
    S: Debug + Display + AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        // Collect the headings.
        let headings: Vec<_> = ctx
            .body()
            .descendants()
            // Find all headings of level 2.
            .filter_map(|start| match &*start.data.borrow() {
                ast @ Ast {
                    value: NodeValue::Heading(NodeHeading { level: 2, .. }),
                    ..
                } => Some((ast.clone(), start)),
                _ => None,
            })
            // Descend into their children.
            .map(|(ast, heading)| {
                let collected = heading
                    .descendants()
                    .skip(1)
                    // Filter for text nodes.
                    .filter_map(|child| match &*child.data.borrow() {
                        Ast {
                            value: NodeValue::Text(v),
                            ..
                        } => Some(v.to_owned()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                (ast, collected)
            })
            .collect();

        // Check for unknown sections.
        let unknowns: Vec<_> = headings
            .iter()
            .filter(|(_, f)| !self.0.iter().any(|e| e == &f.as_str()))
            .map(|(ast, _)| ctx.ast_snippet(ast, None, None))
            .collect();

        if !unknowns.is_empty() {
            ctx.report(
                ctx.annotation_level()
                    .title("body has extra section(s)")
                    .id(slug)
                    .snippets(unknowns),
            )?;
        }

        // Check that sections are in the correct order.
        let map: HashMap<_, _> = headings.into_iter().map(|(a, b)| (b, a)).collect();
        let present: Vec<_> = map.keys().map(String::as_str).collect();

        let mut max_line = 0;
        for name in self.0.iter() {
            let name = name.as_ref();
            if let Some(ast) = map.get(name) {
                let cur = max_line;
                max_line = ast.sourcepos.start.line;

                if max_line >= cur {
                    continue;
                }

                let label = format!("section `{}` is out of order", name);
                let mut footer_label = String::new();
                let mut footer = vec![];

                if let Some(preceding) = self.find_preceding(&present, name) {
                    write!(footer_label, "`{}` should come after `{}`", name, preceding).unwrap();

                    footer.push(Level::Help.title(&footer_label));
                }

                ctx.report(
                    ctx.annotation_level()
                        .title(&label)
                        .id(slug)
                        .footers(footer)
                        .snippet(ctx.ast_snippet(ast, None, None)),
                )?;
            }
        }

        Ok(())
    }
}
