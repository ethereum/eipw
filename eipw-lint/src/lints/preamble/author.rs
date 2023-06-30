/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use regex::RegexSet;

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

fn footer() -> Vec<Annotation<'static>> {
    vec![
        Annotation {
            annotation_type: AnnotationType::Help,
            id: None,
            label: Some("Try `Random J. User (@username) <test@example.com>` for an author with a GitHub username plus email."),
        },
        Annotation {
            annotation_type: AnnotationType::Help,
            id: None,
            label: Some("Try `Random J. User (@username)` for an author with a GitHub username."),
        },
        Annotation {
            annotation_type: AnnotationType::Help,
            id: None,
            label: Some("Try `Random J. User <test@example.com>` for an author with an email."),
        },
        Annotation {
            annotation_type: AnnotationType::Help,
            id: None,
            label: Some("Try `Random J. User` for an author without contact information."),
        },
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Author<S>(pub S);

impl<S> Lint for Author<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
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
            let item_count = item.chars().count();
            offset += item_count + 1;
            let trimmed = item.trim();

            let matches = set.matches(trimmed);

            if matches.matched_any() {
                has_username |= matches.matched(0);
                has_username |= matches.matched(2);
                continue;
            }

            let name_count = field.name().chars().count();

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: ctx.annotation_type(),
                    id: Some(slug),
                    label: Some("authors in the preamble must match the expected format"),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: vec![SourceAnnotation {
                        annotation_type: ctx.annotation_type(),
                        label: "unrecognized author",
                        range: (
                            name_count + current + 1,
                            name_count + current + 1 + item_count,
                        ),
                    }],
                }],
                footer: footer(),
                opt: Default::default(),
            })?;
        }

        if !has_username {
            let label = format!(
                "preamble header `{}` must contain at least one GitHub username",
                self.0
            );
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
                    annotations: vec![],
                }],
                footer: vec![],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
