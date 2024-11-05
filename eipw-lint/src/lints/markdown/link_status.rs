/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Level;

use comrak::nodes::{Ast, AstNode, NodeValue};

use crate::lints::{Context, Error, FetchContext, Lint};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStatus<S> {
    pub status: S,
    pub flow: Vec<Vec<S>>,
    pub pattern: S,
}

impl<S> LinkStatus<S>
where
    S: AsRef<str>,
{
    fn tier(&self, map: &HashMap<&str, usize>, ctx: &Context<'_, '_>) -> usize {
        ctx.preamble()
            .by_name(self.status.as_ref())
            .map(|f| f.value())
            .map(str::trim)
            .and_then(|s| map.get(s))
            .copied()
            .unwrap_or(0)
    }

    fn find_links<'a>(
        &self,
        node: &'a AstNode<'a>,
    ) -> impl 'a + Iterator<Item = (Ast, u32, String)> {
        let re = Regex::new(self.pattern.as_ref()).unwrap();

        node.descendants()
            // Find all URLs and the lines they appear on.
            .filter_map(|start| match &*start.data.borrow() {
                ast @ Ast {
                    value: NodeValue::Link(link),
                    ..
                } => Some((ast.clone(), link.url.clone())),
                _ => None,
            })
            .filter_map(move |(start_line, url)| {
                re.captures(&url).map(|c| {
                    (
                        start_line,
                        c.get(1)
                            .expect("missing capture group for LinkStatus")
                            .as_str()
                            .parse()
                            .expect("bad numeric regex for LinkStatus"),
                        c.get(0).unwrap().as_str().into(),
                    )
                })
            })
    }
}

impl<S> Lint for LinkStatus<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), Error> {
        self.find_links(ctx.body())
            .map(|x| x.1)
            .collect::<HashSet<_>>()
            .into_iter()
            .for_each(|p| ctx.fetch_proposal(p));

        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let mut map = HashMap::new();
        for (tier, values) in self.flow.iter().enumerate() {
            for value in values {
                map.insert(value.as_ref(), tier + 1);
            }
        }

        let my_tier = self.tier(&map, ctx);
        let mut min = usize::MAX;

        for (ast, number, whole) in self.find_links(ctx.body()) {
            let eip = match ctx.proposal(number) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", whole, e);
                    ctx.report(
                        ctx.annotation_level()
                            .title(&label)
                            .id(slug)
                            .snippet(ctx.ast_snippet(&ast, None, None)),
                    )?;
                    continue;
                }
            };

            let their_tier = self.tier(&map, &eip);

            if their_tier < min {
                min = their_tier;
            }

            if their_tier >= my_tier {
                continue;
            }

            let label = format!(
                "proposal `{}` is not stable enough for a `{}` of `{}`",
                whole,
                self.status,
                ctx.preamble()
                    .by_name(self.status.as_ref())
                    .map(|f| f.value())
                    .unwrap_or("<missing>")
                    .trim(),
            );

            let mut choices = map
                .iter()
                .filter_map(|(v, t)| if *t <= min { Some(v) } else { None })
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            choices.sort();

            let choices = choices.join("`, `");

            let mut footer = vec![];
            let footer_label = format!(
                "because of this link, this proposal's `{}` must be one of: `{}`",
                self.status, choices
            );

            if !choices.is_empty() {
                footer.push(Level::Help.title(&footer_label));
            }

            ctx.report(
                ctx.annotation_level()
                    .title(&label)
                    .id(slug)
                    .snippet(ctx.ast_snippet(&ast, None, None))
                    .footers(footer),
            )?;
        }

        Ok(())
    }
}
