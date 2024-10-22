/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Snippet};

use crate::lints::{Context, Error, FetchContext, Lint};
use crate::{LevelExt, SnippetExt};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiresStatus<S> {
    pub requires: S,
    pub status: S,
    pub flow: Vec<Vec<S>>,
}

impl<S> RequiresStatus<S>
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
}

impl<S> Lint for RequiresStatus<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        field
            .value()
            .split(',')
            .map(str::trim)
            .map(str::parse::<u32>)
            .filter_map(Result::ok)
            .for_each(|p| ctx.fetch_proposal(p));

        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let mut map = HashMap::new();
        for (tier, values) in self.flow.iter().enumerate() {
            for value in values.iter() {
                let value = value.as_ref();
                map.insert(value, tier + 1);
            }
        }

        let my_tier = self.tier(&map, ctx);
        let mut too_unstable = Vec::new();
        let mut min = usize::MAX;

        let items = field.value().split(',');

        let mut offset = 0;
        for item in items {
            let name_count = field.name().len();
            let item_count = item.len();

            let current = offset;
            offset += item_count + 1;

            let key = match item.trim().parse::<u32>() {
                Ok(k) => k,
                _ => continue,
            };

            let eip = match ctx.proposal(key) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read proposal number `{}`: {}", key, e);
                    ctx.report(
                        ctx.annotation_level().title(&label).id(slug).snippet(
                            Snippet::source(field.source())
                                .fold(false)
                                .line_start(field.line_start())
                                .origin_opt(ctx.origin())
                                .annotation(
                                    ctx.annotation_level()
                                        .span_utf8(
                                            field.source(),
                                            name_count + current + 1,
                                            item_count,
                                        )
                                        .label("required from here"),
                                ),
                        ),
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

            too_unstable.push(
                ctx.annotation_level()
                    .span_utf8(field.source(), name_count + current + 1, item_count)
                    .label("has a less advanced status"),
            );
        }

        if !too_unstable.is_empty() {
            let label = format!(
                "preamble header `{}` contains items not stable enough for a `{}` of `{}`",
                self.requires,
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
                "valid `{}` values for this proposal are: `{}`",
                self.status, choices
            );

            if !choices.is_empty() {
                footer.push(Level::Help.title(&footer_label));
            }

            ctx.report(
                ctx.annotation_level()
                    .title(&label)
                    .id(slug)
                    .snippet(
                        Snippet::source(field.source())
                            .fold(false)
                            .line_start(field.line_start())
                            .origin_opt(ctx.origin())
                            .annotations(too_unstable),
                    )
                    .footers(footer),
            )?;
        }

        Ok(())
    }
}
