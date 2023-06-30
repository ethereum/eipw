/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Includes,
    Excludes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Regex<S> {
    pub name: S,
    pub mode: Mode,
    pub pattern: S,
    pub message: S,
}

impl<S> Lint for Regex<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let value = field.value().trim();

        let re = ::regex::Regex::new(self.pattern.as_ref()).map_err(Error::custom)?;
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

        let name_count = field.name().chars().count();
        let value_count = field.value().chars().count();

        ctx.report(Snippet {
            title: Some(Annotation {
                annotation_type: ctx.annotation_type(),
                id: Some(slug),
                label: Some(self.message.as_ref()),
            }),
            slices: vec![Slice {
                fold: false,
                line_start: field.line_start(),
                origin: ctx.origin(),
                source: field.source(),
                annotations: vec![SourceAnnotation {
                    annotation_type: ctx.annotation_type(),
                    label: slice_label,
                    range: (name_count + 1, value_count + name_count + 1),
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
