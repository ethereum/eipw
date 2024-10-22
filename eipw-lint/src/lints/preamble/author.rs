/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Message, Snippet};

use regex::RegexSet;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

fn footer() -> Vec<Message<'static>> {
    vec![
        Level::Help.title("Try `Random J. User (@username) <test@example.com>` for an author with a GitHub username plus email."),
        Level::Help.title("Try `Random J. User (@username)` for an author with a GitHub username."),
        Level::Help.title("Try `Random J. User <test@example.com>` for an author with an email."),
        Level::Help.title("Try `Random J. User` for an author without contact information."),
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Author<S>(pub S);

impl<S> Lint for Author<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        // TODO: Email addresses are insane, and can probably contain commas,
        //       parentheses, and greater-/less- than symbols. For correctness,
        //       we should switch to a parser that can handle those cases.

        let items = field.value().split(',');

        let set = RegexSet::new([
            // Match a GitHub username.
            r"^[^()<>,@]+ \(@[a-zA-Z\d-]+\)$",
            // Match an email address.
            r"^[^()<>,@]+ <[^@][^>]*@[^>]+\.[^>]+>$",
            // Match a GitHub username plus email address.
            r"^[^()<>,@]+ \(@[a-zA-Z\d-]+\) <[^@][^>]*@[^>]+\.[^>]+>$",
            // Match just a name.
            r"^[^()<>,@]+$",
        ])
        .unwrap();

        let mut has_username = false;
        let mut offset = 0;

        for item in items {
            let current = offset;
            let item_count = item.len();
            offset += item_count + 1;
            let trimmed = item.trim();

            let matches = set.matches(trimmed);

            if matches.matched_any() {
                has_username |= matches.matched(0);
                has_username |= matches.matched(2);
                continue;
            }

            let name_count = field.name().len();

            let start = name_count + current + 1;
            ctx.report(
                ctx.annotation_level()
                    .title("authors in the preamble must match the expected format")
                    .id(slug)
                    .snippet(
                        Snippet::source(field.source())
                            .fold(false)
                            .line_start(field.line_start())
                            .origin_opt(ctx.origin())
                            .annotation(
                                ctx.annotation_level()
                                    .span_utf8(field.source(), start, item_count)
                                    .label("unrecognized author"),
                            ),
                    )
                    .footers(footer()),
            )?;
        }

        if !has_username {
            let label = format!(
                "preamble header `{}` must contain at least one GitHub username",
                self.0
            );
            ctx.report(
                ctx.annotation_level().title(&label).id(slug).snippet(
                    Snippet::source(field.source())
                        .line_start(field.line_start())
                        .origin_opt(ctx.origin())
                        .fold(false),
                ),
            )?;
        }

        Ok(())
    }
}
