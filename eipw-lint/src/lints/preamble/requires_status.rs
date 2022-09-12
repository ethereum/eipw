/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, FetchContext, Lint};

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RequiresStatus<'n> {
    pub requires: &'n str,
    pub status: &'n str,
    pub flow: &'n [&'n [&'n str]],
}

impl<'n> RequiresStatus<'n> {
    fn tier(&self, map: &HashMap<&str, usize>, ctx: &Context<'_, '_>) -> usize {
        ctx.preamble()
            .by_name(self.status)
            .map(|f| f.value())
            .map(str::trim)
            .and_then(|s| map.get(s))
            .copied()
            .unwrap_or(0)
    }
}

impl<'n> Lint for RequiresStatus<'n> {
    fn find_resources<'a>(&self, ctx: &FetchContext<'a>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires) {
            None => return Ok(()),
            Some(s) => s,
        };

        field
            .value()
            .split(',')
            .map(str::trim)
            .map(str::parse::<u64>)
            .filter_map(Result::ok)
            .map(|n| format!("eip-{}.md", n))
            .map(PathBuf::from)
            .for_each(|p| ctx.fetch(p));

        Ok(())
    }

    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires) {
            None => return Ok(()),
            Some(s) => s,
        };

        let mut map = HashMap::new();
        for (tier, values) in self.flow.iter().enumerate() {
            for value in *values {
                map.insert(*value, tier + 1);
            }
        }

        let my_tier = self.tier(&map, ctx);
        let mut too_unstable = Vec::new();
        let mut min = usize::MAX;

        let items = field.value().split(',');

        let mut offset = 0;
        for item in items {
            let current = offset;
            offset += item.len() + 1;

            let key = match item.trim().parse::<u64>() {
                Ok(k) => PathBuf::from(format!("eip-{}.md", k)),
                _ => continue,
            };

            let eip = match ctx.eip(&key) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", key.display(), e);
                    ctx.report(Snippet {
                        title: Some(Annotation {
                            id: Some(slug),
                            label: Some(&label),
                            annotation_type: ctx.annotation_type(),
                        }),
                        slices: vec![Slice {
                            fold: false,
                            line_start: field.line_start(),
                            origin: ctx.origin(),
                            source: field.source(),
                            annotations: vec![SourceAnnotation {
                                annotation_type: ctx.annotation_type(),
                                label: "required from here",
                                range: (
                                    field.name().len() + current + 1,
                                    field.name().len() + current + 1 + item.len(),
                                ),
                            }],
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

            too_unstable.push(SourceAnnotation {
                annotation_type: ctx.annotation_type(),
                label: "has a less advanced status",
                range: (
                    field.name().len() + current + 1,
                    field.name().len() + current + 1 + item.len(),
                ),
            });
        }

        if !too_unstable.is_empty() {
            let label = format!(
                "preamble header `{}` contains items not stable enough for a `{}` of `{}`",
                self.requires,
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
                "valid `{}` values for this proposal are: `{}`",
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
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: too_unstable,
                }],
                footer,
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
