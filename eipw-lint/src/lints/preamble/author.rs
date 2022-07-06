/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use regex::RegexSet;

use crate::lints::{Context, Error, Lint};

fn footer() -> Vec<Annotation<'static>> {
    vec![
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

#[derive(Debug)]
pub struct Author<'n>(pub &'n str);

impl<'n> Lint for Author<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0) {
            None => return Ok(()),
            Some(s) => s,
        };

        // TODO: Email addresses are insane, and can probably contain commas,
        //       parentheses, and greater-/less- than symbols. For correctness,
        //       we should switch to a parser that can handle those cases.

        let items = field.value().split(',');

        let set = RegexSet::new(&[
            r"^[^()<>,@]+ \(@[a-zA-Z\d-]+\)$", // Match a GitHub username.
            r"^[^()<>,@]+ <[^@][^>]*@[^>]+\.[^>]+>$", // Match an email address.
            r"^[^()<>,@]+$",                   // Match just a name.
        ])
        .unwrap();

        let mut has_username = false;
        let mut offset = 0;

        for item in items {
            let current = offset;
            offset += item.len() + 1;
            let trimmed = item.trim();

            let matches = set.matches(trimmed);

            if matches.matched_any() {
                has_username |= matches.matched(0);
                continue;
            }

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(slug),
                    label: Some("authors in the preamble must match the expected format"),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: vec![SourceAnnotation {
                        annotation_type: AnnotationType::Error,
                        label: "unrecognized author",
                        range: (
                            field.name().len() + current + 1,
                            field.name().len() + current + 1 + item.len(),
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
                    annotation_type: AnnotationType::Error,
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
