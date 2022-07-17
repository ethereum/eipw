/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use regex::Regex;

#[derive(Debug)]
pub struct RequireReferenced<'n> {
    pub name: &'n str,
    pub requires: &'n str,
}

impl<'n> Lint for RequireReferenced<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name) {
            None => return Ok(()),
            Some(f) => f,
        };

        let requires_txt = ctx
            .preamble()
            .by_name(self.requires)
            .map(|f| f.value())
            .unwrap_or_default();

        let requires: Vec<_> = requires_txt
            .split(',')
            .map(str::trim)
            .map(str::parse::<u64>)
            .filter_map(Result::ok)
            .collect();

        let re = Regex::new(r"(?i)eip-([0-9]+)").unwrap();

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

        let annotations = missing
            .iter()
            .map(|m| SourceAnnotation {
                range: (
                    m.start() + field.name().len() + 1,
                    m.end() + field.name().len() + 1,
                ),
                label: "mentioned here",
                annotation_type: AnnotationType::Error,
            })
            .collect();

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
                annotations,
                source: field.source(),
            }],
            ..Default::default()
        })?;

        Ok(())
    }
}
