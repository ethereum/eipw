/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod lints;
pub mod preamble;
pub mod reporters;

use annotate_snippets::snippet::{Annotation, AnnotationType, Snippet};

use comrak::{Arena, ComrakExtensionOptions, ComrakOptions};

use crate::lints::{Context, Error as LintError, Lint, LintExt as _};
use crate::preamble::Preamble;
use crate::reporters::Reporter;

use std::collections::HashMap;

pub fn default_lints() -> impl Iterator<Item = (&'static str, Box<dyn Lint>)> {
    use lints::{markdown, preamble};

    [
        //
        // Preamble
        //
        ("preamble-no-dup", preamble::NoDuplicates.boxed()),
        ("preamble-trim", preamble::Trim.boxed()),
        ("preamble-eip", preamble::Uint("eip").boxed()),
        ("preamble-author", preamble::Author("author").boxed()),
        (
            "preamble-discussions-to",
            preamble::Url("discussions-to").boxed(),
        ),
        ("preamble-list-author", preamble::List("author").boxed()),
        ("preamble-list-requires", preamble::List("requires").boxed()),
        (
            "preamble-uint-requires",
            preamble::UintList("requires").boxed(),
        ),
        (
            "preamble-len-title",
            preamble::Length {
                name: "title",
                min: Some(2),
                max: Some(44),
            }
            .boxed(),
        ),
        (
            "preamble-len-description",
            preamble::Length {
                name: "description",
                min: Some(2),
                max: Some(140),
            }
            .boxed(),
        ),
        (
            "preamble-req",
            preamble::Required(&[
                "eip",
                "title",
                "description",
                "author",
                "discussions-to",
                "status",
                "type",
                "created",
            ])
            .boxed(),
        ),
        (
            "preamble-order",
            preamble::Order(&[
                "eip",
                "title",
                "description",
                "author",
                "discussions-to",
                "status",
                "last-call-deadline",
                "type",
                "category",
                "created",
                "requires",
                "withdrawal-reason",
            ])
            .boxed(),
        ),
        ("preamble-date-created", preamble::Date("created").boxed()),
        (
            "preamble-req-last-call-deadline",
            preamble::RequiredIfEq {
                when: "status",
                equals: "Last Call",
                then: "last-call-deadline",
            }
            .boxed(),
        ),
        (
            "preamble-date-last-call-deadline",
            preamble::Date("last-call-deadline").boxed(),
        ),
        (
            "preamble-req-category",
            preamble::RequiredIfEq {
                when: "type",
                equals: "Standards Track",
                then: "category",
            }
            .boxed(),
        ),
        (
            "preamble-req-withdrawal-reason",
            preamble::RequiredIfEq {
                when: "status",
                equals: "Withdrawn",
                then: "withdrawal-reason",
            }
            .boxed(),
        ),
        (
            "preamble-enum-status",
            preamble::OneOf {
                name: "status",
                values: &[
                    "Draft",
                    "Review",
                    "Last Call",
                    "Final",
                    "Stagnant",
                    "Withdrawn",
                    "Living",
                ],
            }
            .boxed(),
        ),
        (
            "preamble-enum-type",
            preamble::OneOf {
                name: "type",
                values: &["Standards Track", "Meta", "Informational"],
            }
            .boxed(),
        ),
        (
            "preamble-enum-category",
            preamble::OneOf {
                name: "category",
                values: &["Core", "Networking", "Interface", "ERC"],
            }
            .boxed(),
        ),
        //
        // Markdown
        //
        (
            "markdown-order-section",
            markdown::SectionOrder(&[
                "Abstract",
                "Motivation",
                "Specification",
                "Rationale",
                "Backwards Compatibility",
                "Test Cases",
                "Reference Implementation",
                "Security Considerations",
                "Copyright",
            ])
            .boxed(),
        ),
        ("markdown-rel-links", markdown::RelativeLinks.boxed()),
    ]
    .into_iter()
}

#[derive(Debug)]
#[must_use]
pub struct Linter<'a, R> {
    lints: HashMap<&'a str, Box<dyn Lint>>,
    origin: Option<&'a str>,
    reporter: R,
}

impl<'a, R> Default for Linter<'a, R>
where
    R: Default,
{
    fn default() -> Self {
        Self::new(R::default())
    }
}

impl<'a, R> Linter<'a, R> {
    pub fn new(reporter: R) -> Self {
        Self {
            reporter,
            origin: None,
            lints: default_lints().collect(),
        }
    }

    pub fn add_lint<T>(mut self, slug: &'a str, lint: T) -> Self
    where
        T: 'static + Lint,
    {
        if self.lints.insert(slug, lint.boxed()).is_some() {
            panic!("duplicate slug: {}", slug);
        }

        self
    }

    pub fn remove_lint(mut self, slug: &str) -> Self {
        if self.lints.remove(slug).is_none() {
            panic!("no lint with the slug: {}", slug);
        }

        self
    }

    pub fn clear_lints(mut self) -> Self {
        self.lints.clear();
        self
    }

    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
        self
    }
}

impl<'a, R> Linter<'a, R>
where
    R: Reporter,
{
    pub async fn check(self, source: &'a str) -> Result<R, LintError> {
        if self.lints.is_empty() {
            panic!("no lints activated");
        }

        let (preamble_source, body_source) = match Preamble::split(source) {
            Ok(v) => v,
            Err(preamble::SplitError::MissingStart { .. })
            | Err(preamble::SplitError::LeadingGarbage { .. }) => {
                self.reporter.report(Snippet {
                    title: Some(Annotation {
                        id: None,
                        label: Some("first line must be `---` exactly"),
                        annotation_type: AnnotationType::Error,
                    }),
                    ..Default::default()
                })?;
                return Ok(self.reporter);
            }
            Err(preamble::SplitError::MissingEnd { .. }) => {
                self.reporter.report(Snippet {
                    title: Some(Annotation {
                        id: None,
                        label: Some("preamble must be followed by a line containing `---` exactly"),
                        annotation_type: AnnotationType::Error,
                    }),
                    ..Default::default()
                })?;
                return Ok(self.reporter);
            }
        };

        let preamble = match Preamble::parse(self.origin, preamble_source) {
            Ok(p) => p,
            Err(e) => {
                for snippet in e.into_errors() {
                    self.reporter.report(snippet)?;
                }
                Preamble::default()
            }
        };

        let arena = Arena::new();
        let options = ComrakOptions {
            extension: ComrakExtensionOptions {
                table: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut preamble_lines: u32 = preamble_source.matches('\n').count().try_into().unwrap();
        preamble_lines += 3;

        let body = comrak::parse_document(&arena, body_source, &options);

        for node in body.descendants() {
            let mut data = node.data.borrow_mut();
            if data.start_line == 0 {
                if let Some(parent) = node.parent() {
                    // TODO: Check if this actually works; I haven't tested it.
                    data.start_line = parent.data.borrow().start_line;
                }
            } else {
                data.start_line += preamble_lines;
            }
        }

        let context = Context {
            body,
            source,
            body_source,
            preamble,
            origin: self.origin,
            reporter: &self.reporter,
        };

        let mut lints: Vec<_> = self.lints.iter().collect();
        lints.sort_by_key(|l| l.0);

        for (slug, lint) in lints.into_iter() {
            lint.lint(slug, &context)?;
        }

        Ok(self.reporter)
    }
}
