/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RequiredIfEq<S> {
    pub when: S,
    pub equals: S,
    pub then: S,
}

impl<S> Lint for RequiredIfEq<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let then_opt = ctx.preamble().by_name(self.then.as_ref());
        let when_opt = ctx.preamble().by_name(self.when.as_ref());

        let equals = self.equals.as_ref();

        match (when_opt, then_opt) {
            // Correct.
            (None, None) => (),

            // Correct.
            (Some(when), Some(_)) if when.value().trim() == equals => (),

            // Correct.
            (Some(when), None) if when.value().trim() != equals => (),

            // Incorrect.
            (Some(when), None) => {
                let label = format!(
                    "preamble header `{}` is required when `{}` is `{}`",
                    self.then, self.when, self.equals,
                );
                ctx.report(Snippet {
                    title: Some(Annotation {
                        annotation_type: ctx.annotation_type(),
                        id: Some(slug),
                        label: Some(&label),
                    }),
                    footer: vec![],
                    slices: vec![Slice {
                        line_start: when.line_start(),
                        fold: false,
                        origin: ctx.origin(),
                        source: when.source(),
                        annotations: vec![SourceAnnotation {
                            annotation_type: AnnotationType::Info,
                            label: "defined here",
                            range: (0, when.source().chars().count()),
                        }],
                    }],
                    opt: Default::default(),
                })?;
            }

            // Incorrect.
            (Some(when), Some(then)) => {
                let label = format!(
                    "preamble header `{}` is only allowed when `{}` is `{}`",
                    self.then, self.when, self.equals,
                );

                let info_label = format!("unless equal to `{}`", self.equals);

                let mut slices = vec![
                    Slice {
                        line_start: when.line_start(),
                        fold: false,
                        origin: ctx.origin(),
                        source: when.source(),
                        annotations: vec![SourceAnnotation {
                            annotation_type: AnnotationType::Info,
                            label: &info_label,
                            range: (0, when.source().chars().count()),
                        }],
                    },
                    Slice {
                        line_start: then.line_start(),
                        fold: false,
                        origin: ctx.origin(),
                        source: then.source(),
                        annotations: vec![SourceAnnotation {
                            annotation_type: ctx.annotation_type(),
                            label: "remove this",
                            range: (0, then.source().chars().count()),
                        }],
                    },
                ];

                slices.sort_by_key(|s| s.line_start);

                ctx.report(Snippet {
                    title: Some(Annotation {
                        annotation_type: ctx.annotation_type(),
                        id: Some(slug),
                        label: Some(&label),
                    }),
                    footer: vec![],
                    slices,
                    opt: Default::default(),
                })?;
            }

            // Incorrect.
            (None, Some(then)) => {
                let label = format!(
                    "preamble header `{}` is only allowed when `{}` is `{}`",
                    self.then, self.when, self.equals,
                );

                ctx.report(Snippet {
                    title: Some(Annotation {
                        annotation_type: ctx.annotation_type(),
                        id: Some(slug),
                        label: Some(&label),
                    }),
                    footer: vec![],
                    slices: vec![Slice {
                        line_start: then.line_start(),
                        fold: false,
                        origin: ctx.origin(),
                        source: then.source(),
                        annotations: vec![SourceAnnotation {
                            annotation_type: ctx.annotation_type(),
                            label: "defined here",
                            range: (0, then.source().chars().count()),
                        }],
                    }],
                    opt: Default::default(),
                })?;
            }
        }

        Ok(())
    }
}
