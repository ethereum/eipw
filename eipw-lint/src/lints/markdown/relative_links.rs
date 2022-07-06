/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeValue};

use crate::lints::{Context, Error, Lint};

use regex::bytes::Regex;

#[derive(Debug)]
pub struct RelativeLinks;

impl Lint for RelativeLinks {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let re = Regex::new("(^/)|(://)").unwrap();

        let links = ctx
            .body()
            .descendants()
            // Find all URLs and the lines they appear on.
            .filter_map(|start| match &*start.data.borrow() {
                Ast {
                    value: NodeValue::Image(link),
                    start_line,
                    ..
                } => Some((*start_line, link.url.clone())),
                Ast {
                    value: NodeValue::Link(link),
                    start_line,
                    ..
                } => Some((*start_line, link.url.clone())),
                _ => None,
            })
            .filter(|(_, url)| re.is_match(url));

        for (line_start, _) in links {
            ctx.report(Snippet {
                title: Some(Annotation {
                    id: Some(slug),
                    annotation_type: AnnotationType::Error,
                    label: Some("non-relative link or image"),
                }),
                footer: vec![],
                slices: vec![Slice {
                    line_start: usize::try_from(line_start).unwrap(),
                    fold: false,
                    origin: ctx.origin(),
                    source: ctx.line(line_start),
                    annotations: vec![],
                }],
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
