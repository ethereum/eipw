/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, AstNode, NodeValue};

use crate::lints::{Context, Error, FetchContext, Lint};

use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug)]
pub struct LinkStatus<'n> {
    pub status: &'n str,
    pub flow: &'n [&'n [&'n str]],
}

impl<'n> LinkStatus<'n> {
    fn tier(&self, map: &HashMap<&str, usize>, ctx: &Context<'_, '_>) -> usize {
        ctx.preamble()
            .by_name(self.status)
            .map(|f| f.value())
            .map(str::trim)
            .and_then(|s| map.get(s))
            .copied()
            .unwrap_or(0)
    }

    fn find_links<'a>(node: &'a AstNode<'a>) -> impl 'a + Iterator<Item = (usize, PathBuf)> {
        let re = Regex::new("(?i)eip-([0-9]+).md$").unwrap();

        node.descendants()
            // Find all URLs and the lines they appear on.
            .filter_map(|start| match &*start.data.borrow() {
                Ast {
                    value: NodeValue::Link(link),
                    sourcepos,
                    ..
                } => Some((sourcepos.start.line, link.url.clone())),
                _ => None,
            })
            .filter_map(move |(start_line, url)| {
                // This is a bit of a cheat, honestly. Doesn't correctly respect directories, but
                // also doesn't allow directory traversal.
                re.captures(&url).map(|c| {
                    (
                        start_line,
                        format!("eip-{}.md", c.get(1).unwrap().as_str()).into(),
                    )
                })
            })
    }
}

impl<'n> Lint for LinkStatus<'n> {
    fn find_resources<'a>(&self, ctx: &FetchContext<'a>) -> Result<(), Error> {
        Self::find_links(ctx.body())
            .map(|x| x.1)
            .collect::<HashSet<_>>()
            .into_iter()
            .for_each(|p| ctx.fetch(p));

        Ok(())
    }

    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let mut map = HashMap::new();
        for (tier, values) in self.flow.iter().enumerate() {
            for value in *values {
                map.insert(*value, tier + 1);
            }
        }

        let my_tier = self.tier(&map, ctx);
        let mut min = usize::MAX;

        for (start_line, url) in Self::find_links(ctx.body()) {
            let eip = match ctx.eip(&url) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", url.display(), e);
                    ctx.report(Snippet {
                        title: Some(Annotation {
                            id: Some(slug),
                            label: Some(&label),
                            annotation_type: ctx.annotation_type(),
                        }),
                        slices: vec![Slice {
                            fold: false,
                            line_start: start_line,
                            origin: ctx.origin(),
                            source: ctx.line(start_line),
                            annotations: vec![],
                        }],
                        ..Default::default()
                    })?;
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
                url.display(),
                self.status,
                ctx.preamble()
                    .by_name(self.status)
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
                footer.push(Annotation {
                    annotation_type: AnnotationType::Help,
                    id: None,
                    label: Some(&footer_label),
                });
            }

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: start_line,
                    origin: ctx.origin(),
                    source: ctx.line(start_line),
                    annotations: vec![],
                }],
                footer,
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
