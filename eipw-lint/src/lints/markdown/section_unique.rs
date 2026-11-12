/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct SectionUnique;

impl Lint for SectionUnique {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let headings: Vec<_> = ctx
            .body()
            .descendants()
            .filter_map(|node| match &*node.data.borrow() {
                ast @ Ast {
                    value: NodeValue::Heading(NodeHeading { level: 2, .. }),
                    ..
                } => Some((ast.clone(), node)),
                _ => None,
            })
            .map(|(ast, heading)| {
                let text = heading
                    .descendants()
                    .skip(1)
                    .filter_map(|child| match &*child.data.borrow() {
                        Ast {
                            value: NodeValue::Text(value),
                            ..
                        } => Some(value.to_owned()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                (ast, text)
            })
            .collect();

        let mut first_by_name: HashMap<String, Ast> = HashMap::new();

        for (ast, name) in headings {
            if let Some(first) = first_by_name.get(&name) {
                let title = format!("duplicate section `{name}`");
                ctx.report(ctx.annotation_level().title(&title).id(slug).snippets([
                    ctx.ast_snippet(first, None, Some("first occurrence")),
                    ctx.ast_snippet(&ast, None, Some("duplicate occurrence")),
                ]))?;
            } else {
                first_by_name.insert(name, ast);
            }
        }

        Ok(())
    }
}
