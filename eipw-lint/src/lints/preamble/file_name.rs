/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::{Level, Snippet};

use crate::lints::{Context, Error, Lint};
use crate::{LevelExt, SnippetExt};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FileName<S> {
    pub name: S,
    pub prefix: S,
    pub suffix: S,
}

impl<S> Lint for FileName<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let file_name = match ctx.origin() {
            None => return Ok(()),
            Some(o) => Path::new(o)
                .file_name()
                .expect("origin did not have a file name"),
        };

        let expected = format!("{}{}{}", self.prefix, field.value().trim(), self.suffix);

        if file_name == expected.as_str() {
            return Ok(());
        }

        let label = format!("file name must reflect the preamble header `{}`", self.name);
        let footer_label = format!("this file's name should be `{}`", expected);

        let name_count = field.name().len();
        let value_count = field.value().len();

        ctx.report(
            ctx.annotation_level()
                .title(&label)
                .id(slug)
                .snippet(
                    Snippet::source(field.source())
                        .fold(false)
                        .line_start(field.line_start())
                        .origin_opt(ctx.origin())
                        .annotation(
                            ctx.annotation_level()
                                .span_utf8(field.source(), name_count + 1, value_count)
                                .label("this value"),
                        ),
                )
                .footer(Level::Help.title(&footer_label)),
        )?;

        Ok(())
    }
}
