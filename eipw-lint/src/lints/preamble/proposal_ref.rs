/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, FetchContext, Lint};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProposalRef<S>(pub S);

impl<S> ProposalRef<S> {
    fn regex() -> Regex {
        // NB: This regex is used to calculate a path, so be careful of directory traversal.
        Regex::new(r"(?i)\b(?:eip|erc)-([0-9]+)\b").unwrap()
    }
}

impl<S> Lint for ProposalRef<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        Self::regex()
            .captures_iter(field.value())
            .map(|x| x.get(1).unwrap().as_str())
            .map(|x| x.parse::<u64>().unwrap())
            .map(|n| format!("eip-{}.md", n))
            .for_each(|p| ctx.fetch(p.into()));

        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let regex = Self::regex();
        let captures = regex.captures_iter(field.value());

        let name_count = field.name().chars().count();

        for capture in captures {
            let whole = capture.get(0).unwrap();

            let start_text = &field.value()[..whole.start()];
            let start = start_text.chars().count() + name_count + 1;

            let end_text = &field.value()[..whole.end()];
            let end = end_text.chars().count() + name_count + 1;

            let number = capture.get(1).unwrap();
            let url = format!("eip-{}.md", number.as_str());

            let eip = match ctx.eip(Path::new(&url)) {
                Ok(eip) => eip,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", url, e);
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
                                label: "referenced here",
                                range: (start, end),
                            }],
                        }],
                        ..Default::default()
                    })?;
                    continue;
                }
            };

            let category = eip.preamble().by_name("category").map(|f| f.value().trim());

            let prefix = match category {
                Some("ERC") => "ERC",
                _ => "EIP",
            };

            if whole.as_str().starts_with(prefix) {
                continue;
            }

            let category_msg = match category {
                Some(c) => format!("with a `category` of `{}`", c),
                None => "without a `category`".to_string(),
            };

            let label = format!(
                "references to proposals {} must use a prefix of `{}`",
                category_msg, prefix,
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
                    annotations: vec![SourceAnnotation {
                        annotation_type: ctx.annotation_type(),
                        label: "referenced here",
                        range: (start, end),
                    }],
                }],
                ..Default::default()
            })?;
        }

        Ok(())
    }
}
