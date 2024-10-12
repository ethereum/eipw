/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

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

        let name_count = field.name().len();

        let annotations = missing.iter().map(|m| {
            let start = field.value()[..m.start()].len() + name_count + 1;
            ctx.annotation_level()
                .span_utf8(field.source(), start, m.end() - m.start())
                .label("mentioned here")
        });

        ctx.report(
            ctx.annotation_level().title(&label).id(slug).snippet(
                Snippet::source(field.source())
                    .annotations(annotations)
                    .fold(false)
                    .line_start(field.line_start())
                    .origin_opt(ctx.origin()),
            ),
        )?;

        Ok(())
    }
}
