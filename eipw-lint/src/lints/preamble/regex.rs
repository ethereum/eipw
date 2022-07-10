/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum Mode {
    Includes,
    Excludes,
}

#[derive(Debug)]
pub struct Regex<'n> {
    pub name: &'n str,
    pub mode: Mode,
    pub pattern: &'n str,
    pub message: &'n str,
}

impl<'n> Lint for Regex<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name) {
            None => return Ok(()),
            Some(s) => s,
        };

        let value = field.value().trim();

        let re = ::regex::Regex::new(self.pattern).map_err(Error::custom)?;
        let matches = re.is_match(value);

        let slice_label = match (self.mode, matches) {
            (Mode::Includes, true) => return Ok(()),
            (Mode::Excludes, false) => return Ok(()),

            (Mode::Includes, false) => "required pattern was not matched",
            (Mode::Excludes, true) => "prohibited pattern was matched",
        };

        let footer_label = format!("the pattern in question: `{}`", self.pattern);

        // TODO: Actually highlight the matches for `Mode::Excludes`, and not
        //       just the whole value.

        ctx.report(Snippet {
            title: Some(Annotation {
                annotation_type: AnnotationType::Error,
                id: Some(slug),
                label: Some(self.message),
            }),
            slices: vec![Slice {
                fold: false,
                line_start: field.line_start(),
                origin: ctx.origin(),
                source: field.source(),
                annotations: vec![SourceAnnotation {
                    annotation_type: AnnotationType::Error,
                    label: slice_label,
                    range: (
                        field.name().len() + 1,
                        field.value().len() + field.name().len() + 1,
                    ),
                }],
            }],
            footer: vec![Annotation {
                id: None,
                annotation_type: AnnotationType::Info,
                label: Some(&footer_label),
            }],
            opt: Default::default(),
        })?;

        Ok(())
    }
}
