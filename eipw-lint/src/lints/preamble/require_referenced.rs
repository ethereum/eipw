/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RequireReferenced<S> {
    pub name: S,
    pub requires: S,
}

impl<S> Lint for RequireReferenced<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(f) => f,
        };

        let requires_txt = ctx
            .preamble()
            .by_name(self.requires.as_ref())
            .map(|f| f.value())
            .unwrap_or_default();

        let requires: Vec<_> = requires_txt
            .split(',')
            .map(str::trim)
            .map(str::parse::<u64>)
            .filter_map(Result::ok)
            .collect();

        let re = Regex::new(r"(?i)(?:eip|erc)-([0-9]+)").unwrap();

        let missing: Vec<_> = re
            .captures_iter(field.value())
            .filter_map(|m| {
                let number: u64 = m[1].parse().unwrap();
                if requires.contains(&number) {
                    None
                } else {
                    Some(m.get(0).unwrap())
                }
            })
            .collect();

        if missing.is_empty() {
            return Ok(());
        }

        let label = format!(
            "proposals mentioned in preamble header `{}` must appear in `{}`",
            self.name, self.requires,
        );

        let name_count = field.name().chars().count();

        let annotations = missing
            .iter()
            .map(|m| SourceAnnotation {
                range: (
                    field.value()[..m.start()].chars().count() + name_count + 1,
                    field.value()[..m.end()].chars().count() + name_count + 1,
                ),
                label: "mentioned here",
                annotation_type: ctx.annotation_type(),
            })
            .collect();

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
                annotations,
                source: field.source(),
            }],
            ..Default::default()
        })?;

        Ok(())
    }
}
