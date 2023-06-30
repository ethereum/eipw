/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod fetch;
pub mod lints;
pub mod modifiers;
pub mod preamble;
pub mod reporters;
pub mod tree;

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::arena_tree::Node;
use comrak::nodes::Ast;
use comrak::{Arena, ComrakExtensionOptions, ComrakOptions};

use crate::lints::{Context, DefaultLint, Error as LintError, FetchContext, InnerContext, Lint};
use crate::modifiers::Modifier;
use crate::preamble::Preamble;
use crate::reporters::Reporter;

use educe::Educe;

use snafu::{ensure, ResultExt, Snafu};

use std::cell::RefCell;
use std::collections::hash_map::{self, HashMap};
use std::path::{Path, PathBuf};

#[derive(Snafu, Debug)]
#[non_exhaustive]
pub enum Error {
    Lint {
        #[snafu(backtrace)]
        source: LintError,
        origin: Option<PathBuf>,
    },
    #[snafu(context(false))]
    Modifier {
        #[snafu(backtrace)]
        source: crate::modifiers::Error,
    },
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    SliceFetched {
        lint: String,
        origin: Option<PathBuf>,
    },
}

fn default_modifiers() -> Vec<Box<dyn Modifier>> {
    vec![
        Box::new(modifiers::SetDefaultAnnotation {
            name: "status",
            value: "Stagnant",
            annotation_type: AnnotationType::Warning,
        }),
        Box::new(modifiers::SetDefaultAnnotation {
            name: "status",
            value: "Withdrawn",
            annotation_type: AnnotationType::Warning,
        }),
    ]
}

pub fn default_lints() -> impl Iterator<Item = (&'static str, Box<dyn Lint>)> {
    default_lints_enum().map(|(name, lint)| (name, lint.boxed()))
}

fn default_lints_enum() -> impl Iterator<Item = (&'static str, DefaultLint<&'static str>)> {
    use self::DefaultLint::*;
    use lints::preamble::regex;
    use lints::{markdown, preamble};

    [
        //
        // Preamble
        //
        ("preamble-no-dup", PreambleNoDuplicates(preamble::NoDuplicates)),
        ("preamble-trim", PreambleTrim(preamble::Trim)),
        ("preamble-eip", PreambleUint { name: preamble::Uint("eip") }),
        ("preamble-author", PreambleAuthor { name: preamble::Author("author") } ),
        ("preamble-re-title", PreambleRegex(preamble::Regex {
            name: "title",
            mode: regex::Mode::Excludes,
            pattern: r"(?i)standar\w*\b",
            message: "preamble header `title` should not contain `standard` (or similar words.)",
        })),
        ("preamble-re-title-colon", PreambleRegex(preamble::Regex {
            name: "title",
            mode: regex::Mode::Excludes,
            pattern: r":",
            message: "preamble header `title` should not contain `:`",
        })),
        (
            "preamble-refs-title",
            PreambleProposalRef { name: preamble::ProposalRef("title") },
        ),
        (
            "preamble-refs-description",
            PreambleProposalRef { name: preamble::ProposalRef("description") },
        ),
        (
            "preamble-re-title-erc-dash",
            PreambleRegex(preamble::Regex {
                name: "title",
                mode: regex::Mode::Excludes,
                pattern: r"(?i)erc[\s]*[0-9]+",
                message: "proposals must be referenced with the form `ERC-N` (not `ERCN` or `ERC N`)",
            }),
        ),
        (
            "preamble-re-title-eip-dash",
            PreambleRegex(preamble::Regex {
                name: "title",
                mode: regex::Mode::Excludes,
                pattern: r"(?i)eip[\s]*[0-9]+",
                message: "proposals must be referenced with the form `EIP-N` (not `EIPN` or `EIP N`)",
            }),
        ),
        (
            "preamble-re-description-erc-dash",
            PreambleRegex(preamble::Regex {
                name: "description",
                mode: regex::Mode::Excludes,
                pattern: r"(?i)erc[\s]*[0-9]+",
                message: "proposals must be referenced with the form `ERC-N` (not `ERCN` or `ERC N`)",
            }),
        ),
        (
            "preamble-re-description-eip-dash",
            PreambleRegex(preamble::Regex {
                name: "description",
                mode: regex::Mode::Excludes,
                pattern: r"(?i)eip[\s]*[0-9]+",
                message: "proposals must be referenced with the form `EIP-N` (not `EIPN` or `EIP N`)",
            }),
        ),
        ("preamble-re-description", PreambleRegex(preamble::Regex {
            name: "description",
            mode: regex::Mode::Excludes,
            pattern: r"(?i)standar\w*\b",
            message: "preamble header `description` should not contain `standard` (or similar words.)",
        })),
        ("preamble-re-description-colon", PreambleRegex(preamble::Regex {
            name: "description",
            mode: regex::Mode::Excludes,
            pattern: r":",
            message: "preamble header `description` should not contain `:`",
        })),
        (
            "preamble-discussions-to",
            PreambleUrl { name: preamble::Url("discussions-to") },
        ),
        (
            "preamble-re-discussions-to",
            PreambleRegex(preamble::Regex {
                name: "discussions-to",
                mode: regex::Mode::Includes,
                pattern: "^https://ethereum-magicians.org/t/[^/]+/[0-9]+$",
                message: concat!(
                    "preamble header `discussions-to` should ",
                    "point to a thread on ethereum-magicians.org"
                ),
            }),
        ),
        ("preamble-list-author", PreambleList { name: preamble::List("author") }),
        ("preamble-list-requires", PreambleList{name: preamble::List("requires")}),
        (
            "preamble-len-requires",
            PreambleLength(preamble::Length {
                name: "requires",
                min: Some(1),
                max: None,
            }
            ),
        ),
        (
            "preamble-uint-requires",
            PreambleUintList { name: preamble::UintList("requires") },
        ),
        (
            "preamble-len-title",
            PreambleLength(preamble::Length {
                name: "title",
                min: Some(2),
                max: Some(44),
            }
            ),
        ),
        (
            "preamble-len-description",
            PreambleLength(preamble::Length {
                name: "description",
                min: Some(2),
                max: Some(140),
            }
            ),
        ),
        (
            "preamble-req",
            PreambleRequired { names: preamble::Required(vec![
                "eip",
                "title",
                "description",
                "author",
                "discussions-to",
                "status",
                "type",
                "created",
            ])
            },
        ),
        (
            "preamble-order",
            PreambleOrder { names: preamble::Order(vec![
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
            },
        ),
        ("preamble-date-created", PreambleDate { name: preamble::Date("created") } ),
        (
            "preamble-req-last-call-deadline",
            PreambleRequiredIfEq(preamble::RequiredIfEq {
                when: "status",
                equals: "Last Call",
                then: "last-call-deadline",
            }
            ),
        ),
        (
            "preamble-date-last-call-deadline",
            PreambleDate { name: preamble::Date("last-call-deadline") },
        ),
        (
            "preamble-req-category",
            PreambleRequiredIfEq(preamble::RequiredIfEq {
                when: "type",
                equals: "Standards Track",
                then: "category",
            }
            ),
        ),
        (
            "preamble-req-withdrawal-reason",
            PreambleRequiredIfEq(preamble::RequiredIfEq {
                when: "status",
                equals: "Withdrawn",
                then: "withdrawal-reason",
            }
            ),
        ),
        (
            "preamble-enum-status",
            PreambleOneOf(preamble::OneOf {
                name: "status",
                values: vec![
                    "Draft",
                    "Review",
                    "Last Call",
                    "Final",
                    "Stagnant",
                    "Withdrawn",
                    "Living",
                ],
            }
            ),
        ),
        (
            "preamble-enum-type",
            PreambleOneOf(preamble::OneOf {
                name: "type",
                values: vec!["Standards Track", "Meta", "Informational"],
            }
            ),
        ),
        (
            "preamble-enum-category",
            PreambleOneOf(preamble::OneOf {
                name: "category",
                values: vec!["Core", "Networking", "Interface", "ERC"],
            }
            ),
        ),
        (
            "preamble-requires-status",
            PreambleRequiresStatus(preamble::RequiresStatus {
                requires: "requires",
                status: "status",
                flow: vec![
                    vec!["Draft", "Stagnant"],
                    vec!["Review"],
                    vec!["Last Call"],
                    vec!["Final", "Withdrawn", "Living"],
                ]
            }),
        ),
        (
            "preamble-requires-ref-title",
            PreambleRequireReferenced(preamble::RequireReferenced {
                name: "title",
                requires: "requires",
            }),
        ),
        (
            "preamble-requires-ref-description",
            PreambleRequireReferenced(preamble::RequireReferenced {
                name: "description",
                requires: "requires",
            }),
        ),
        (
            "preamble-file-name",
            PreambleFileName(preamble::FileName {
                name: "eip",
                prefix: "eip-",
                suffix: ".md",
            }),
        ),
        //
        // Markdown
        //
        (
            "markdown-refs",
            MarkdownProposalRef(markdown::ProposalRef),
        ),
        (
            "markdown-html-comments",
            MarkdownHtmlComments(markdown::HtmlComments {
                name: "status",
                warn_for: vec![
                    "Draft",
                    "Withdrawn",
                ],
            }
            ),
        ),
        (
            "markdown-req-section",
            MarkdownSectionRequired { sections: markdown::SectionRequired(vec![
                "Abstract",
                "Specification",
                "Rationale",
                "Security Considerations",
                "Copyright",
            ])
            },
        ),
        (
            "markdown-order-section",
            MarkdownSectionOrder {
                sections: markdown::SectionOrder(vec![
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
            },
        ),
        (
            "markdown-re-erc-dash",
            MarkdownRegex(markdown::Regex {
                mode: markdown::regex::Mode::Excludes,
                pattern: r"(?i)erc[\s]*[0-9]+",
                message: "proposals must be referenced with the form `ERC-N` (not `ERCN` or `ERC N`)",
            }),
        ),
        (
            "markdown-re-eip-dash",
            MarkdownRegex(markdown::Regex {
                mode: markdown::regex::Mode::Excludes,
                pattern: r"(?i)eip[\s]*[0-9]+",
                message: "proposals must be referenced with the form `EIP-N` (not `EIPN` or `EIP N`)",
            }),
        ),
        (
            "markdown-link-first",
            MarkdownLinkFirst {
                pattern: markdown::LinkFirst(r"(?i)(?:eip|erc)-[0-9]+"),
            }
        ),
        ("markdown-rel-links", MarkdownRelativeLinks(markdown::RelativeLinks {
            exceptions: vec![
                "^https://(www\\.)?github\\.com/ethereum/consensus-specs/blob/[a-f0-9]{40}/.+$",
                "^https://(www\\.)?github\\.com/ethereum/consensus-specs/commit/[a-f0-9]{40}$",

                "^https://(www\\.)?github\\.com/ethereum/devp2p/blob/[0-9a-f]{40}/.+$",
                "^https://(www\\.)?github\\.com/ethereum/devp2p/commit/[0-9a-f]{40}$",

                "^https://(www\\.)?github\\.com/bitcoin/bips/blob/[0-9a-f]{40}/bip-[0-9]+\\.mediawiki$",

                "^https://www\\.w3\\.org/TR/[0-9][0-9][0-9][0-9]/.*$",
                "^https://[a-z]*\\.spec\\.whatwg\\.org/commit-snapshots/[0-9a-f]{40}/$",
                "^https://www\\.rfc-editor\\.org/rfc/.*$",
            ]
        })),
        (
            "markdown-link-status",
            MarkdownLinkStatus(markdown::LinkStatus {
                status: "status",
                flow: vec![
                    vec!["Draft", "Stagnant"],
                    vec!["Review"],
                    vec!["Last Call"],
                    vec!["Final", "Withdrawn", "Living"],
                ]
            }),
        ),
        (
            "markdown-json-cite",
            MarkdownJsonSchema(markdown::JsonSchema {
                additional_schemas: vec![
                    (
                        "https://resource.citationstyles.org/schema/v1.0/input/json/csl-data.json",
                        include_str!("lints/markdown/json_schema/csl-data.json"),
                    ),
                ],
                schema: include_str!("lints/markdown/json_schema/citation.json"),
                language: "csl-json",
                help: concat!(
                    "see https://github.com/ethereum/eipw/blob/",
                    "master/eipw-lint/src/lints/markdown/",
                    "json_schema/citation.json",
                ),
            }),
        ),
    ]
    .into_iter()
}

#[derive(Debug)]
enum Source<'a> {
    String {
        origin: Option<&'a str>,
        src: &'a str,
    },
    File(&'a Path),
}

impl<'a> Source<'a> {
    fn origin(&self) -> Option<&Path> {
        match self {
            Self::String {
                origin: Some(s), ..
            } => Some(Path::new(s)),
            Self::File(p) => Some(p),
            _ => None,
        }
    }

    fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    async fn fetch(&self, fetch: &dyn fetch::Fetch) -> Result<String, Error> {
        match self {
            Self::File(f) => fetch
                .fetch(f.to_path_buf())
                .await
                .with_context(|_| IoSnafu { path: f.to_owned() })
                .map_err(Into::into),
            Self::String { src, .. } => Ok((*src).to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Settings<'a> {
    _p: std::marker::PhantomData<&'a dyn Lint>,
    pub default_annotation_type: AnnotationType,
}

#[derive(Educe)]
#[educe(Debug)]
#[must_use]
pub struct Linter<'a, R> {
    lints: HashMap<&'a str, (Option<AnnotationType>, Box<dyn Lint>)>,
    modifiers: Vec<Box<dyn Modifier>>,
    sources: Vec<Source<'a>>,

    #[educe(Debug(ignore))]
    reporter: R,

    #[educe(Debug(ignore))]
    fetch: Box<dyn fetch::Fetch>,
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
    pub fn with_lints<'b: 'a>(
        reporter: R,
        lints: impl Iterator<Item = (&'b str, Box<dyn Lint>)>,
    ) -> Self {
        Self {
            reporter,
            sources: Default::default(),
            fetch: Box::<fetch::DefaultFetch>::default(),
            modifiers: default_modifiers(),
            lints: lints.map(|(slug, lint)| (slug, (None, lint))).collect(),
        }
    }

    pub fn new(reporter: R) -> Self {
        Self::with_lints(reporter, default_lints())
    }

    pub fn warn<T>(self, slug: &'a str, lint: T) -> Self
    where
        T: 'static + Lint,
    {
        self.add_lint(Some(AnnotationType::Warning), slug, lint)
    }

    pub fn deny<T>(self, slug: &'a str, lint: T) -> Self
    where
        T: 'static + Lint,
    {
        self.add_lint(Some(AnnotationType::Error), slug, lint)
    }

    pub fn modify<T>(mut self, modifier: T) -> Self
    where
        T: 'static + Modifier,
    {
        self.modifiers.push(Box::new(modifier));
        self
    }

    fn add_lint<T>(mut self, level: Option<AnnotationType>, slug: &'a str, lint: T) -> Self
    where
        T: 'static + Lint,
    {
        self.lints.insert(slug, (level, Box::new(lint)));
        self
    }

    pub fn allow(mut self, slug: &str) -> Self {
        if self.lints.remove(slug).is_none() {
            panic!("no lint with the slug: {}", slug);
        }

        self
    }

    pub fn clear_lints(mut self) -> Self {
        self.lints.clear();
        self
    }

    pub fn set_fetch<F>(mut self, fetch: F) -> Self
    where
        F: 'static + fetch::Fetch,
    {
        self.fetch = Box::new(fetch);
        self
    }
}

impl<'a, R> Linter<'a, R>
where
    R: Reporter,
{
    pub fn check_slice(mut self, origin: Option<&'a str>, src: &'a str) -> Self {
        self.sources.push(Source::String { origin, src });
        self
    }

    pub fn check_file(mut self, path: &'a Path) -> Self {
        self.sources.push(Source::File(path));
        self
    }

    pub async fn run(self) -> Result<R, Error> {
        if self.lints.is_empty() {
            panic!("no lints activated");
        }

        if self.sources.is_empty() {
            panic!("no sources given");
        }

        let mut to_check = Vec::with_capacity(self.sources.len());
        let mut fetched_eips = HashMap::new();

        for source in self.sources {
            let source_origin = source.origin().map(Path::to_path_buf);
            let source_content = source.fetch(&*self.fetch).await?;

            to_check.push((source_origin, source_content));

            let (source_origin, source_content) = to_check.last().unwrap();
            let display_origin = source_origin.as_deref().map(Path::to_string_lossy);
            let display_origin = display_origin.as_deref();

            let arena = Arena::new();
            let inner = match process(&reporters::Null, &arena, display_origin, source_content)? {
                Some(i) => i,
                None => continue,
            };

            for (slug, lint) in &self.lints {
                let context = FetchContext {
                    body: inner.body,
                    preamble: &inner.preamble,
                    eips: Default::default(),
                };

                lint.1
                    .find_resources(&context)
                    .with_context(|_| LintSnafu {
                        origin: source_origin.clone(),
                    })?;

                let eips = context.eips.into_inner();

                // For now, string sources shouldn't be allowed to fetch external
                // resources. The origin field isn't guaranteed to be a file/URL,
                // and even if it was, we wouldn't know which of those to interpret
                // it as.
                ensure!(
                    eips.is_empty() || !source.is_string(),
                    SliceFetchedSnafu {
                        lint: *slug,
                        origin: source_origin.clone(),
                    }
                );

                for eip in eips.into_iter() {
                    let root = match source {
                        Source::File(p) => p.parent().unwrap_or_else(|| Path::new(".")),
                        _ => unreachable!(),
                    };

                    let path = root.join(eip);

                    let entry = match fetched_eips.entry(path) {
                        hash_map::Entry::Occupied(_) => continue,
                        hash_map::Entry::Vacant(v) => v,
                    };

                    let content = Source::File(entry.key()).fetch(&*self.fetch).await;
                    entry.insert(content);
                }
            }
        }

        let resources_arena = Arena::new();
        let mut parsed_eips = HashMap::new();

        for (origin, result) in &fetched_eips {
            let source = match result {
                Ok(o) => o,
                Err(e) => {
                    parsed_eips.insert(origin.as_path(), Err(e));
                    continue;
                }
            };

            let inner = match process(&self.reporter, &resources_arena, None, source)? {
                Some(s) => s,
                None => return Ok(self.reporter),
            };
            parsed_eips.insert(origin.as_path(), Ok(inner));
        }

        let mut lints: Vec<_> = self.lints.iter().collect();
        lints.sort_by_key(|l| l.0);

        for (origin, source) in &to_check {
            let display_origin = origin.as_ref().map(|p| p.to_string_lossy().into_owned());
            let display_origin = display_origin.as_deref();

            let arena = Arena::new();
            let inner = match process(&self.reporter, &arena, display_origin, source)? {
                Some(i) => i,
                None => continue,
            };

            let mut settings = Settings {
                _p: std::marker::PhantomData,
                default_annotation_type: AnnotationType::Error,
            };

            for modifier in &self.modifiers {
                let context = Context {
                    inner: inner.clone(),
                    reporter: &self.reporter,
                    eips: &parsed_eips,
                    annotation_type: settings.default_annotation_type,
                };

                modifier.modify(&context, &mut settings)?;
            }

            for (slug, (annotation_type, lint)) in &lints {
                let annotation_type = annotation_type.unwrap_or(settings.default_annotation_type);
                let context = Context {
                    inner: inner.clone(),
                    reporter: &self.reporter,
                    eips: &parsed_eips,
                    annotation_type,
                };

                lint.lint(slug, &context).with_context(|_| LintSnafu {
                    origin: origin.clone(),
                })?;
            }
        }

        Ok(self.reporter)
    }
}

fn process<'r, 'a>(
    reporter: &'r dyn Reporter,
    arena: &'a Arena<Node<'a, RefCell<Ast>>>,
    origin: Option<&'a str>,
    source: &'a str,
) -> Result<Option<InnerContext<'a>>, Error> {
    let (preamble_source, body_source) = match Preamble::split(source) {
        Ok(v) => v,
        Err(preamble::SplitError::MissingStart { .. })
        | Err(preamble::SplitError::LeadingGarbage { .. }) => {
            let mut footer = Vec::new();
            if source.as_bytes().get(3) == Some(&b'\r') {
                footer.push(Annotation {
                    id: None,
                    label: Some(
                        "found a carriage return (CR), use Unix-style line endings (LF) instead",
                    ),
                    annotation_type: AnnotationType::Help,
                });
            }
            reporter
                .report(Snippet {
                    title: Some(Annotation {
                        id: None,
                        label: Some("first line must be `---` exactly"),
                        annotation_type: AnnotationType::Error,
                    }),
                    slices: vec![Slice {
                        fold: false,
                        line_start: 1,
                        origin,
                        source: source.lines().next().unwrap_or_default(),
                        annotations: vec![],
                    }],
                    footer,
                    ..Default::default()
                })
                .map_err(LintError::from)
                .with_context(|_| LintSnafu {
                    origin: origin.map(PathBuf::from),
                })?;
            return Ok(None);
        }
        Err(preamble::SplitError::MissingEnd { .. }) => {
            reporter
                .report(Snippet {
                    title: Some(Annotation {
                        id: None,
                        label: Some("preamble must be followed by a line containing `---` exactly"),
                        annotation_type: AnnotationType::Error,
                    }),
                    ..Default::default()
                })
                .map_err(LintError::from)
                .with_context(|_| LintSnafu {
                    origin: origin.map(PathBuf::from),
                })?;
            return Ok(None);
        }
    };

    let preamble = match Preamble::parse(origin, preamble_source) {
        Ok(p) => p,
        Err(e) => {
            for snippet in e.into_errors() {
                reporter
                    .report(snippet)
                    .map_err(LintError::from)
                    .with_context(|_| LintSnafu {
                        origin: origin.map(PathBuf::from),
                    })?;
            }
            Preamble::default()
        }
    };

    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            table: true,
            autolink: true,
            footnotes: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut preamble_lines = preamble_source.matches('\n').count();
    preamble_lines += 3;

    let body = comrak::parse_document(arena, body_source, &options);

    for node in body.descendants() {
        let mut data = node.data.borrow_mut();
        if data.sourcepos.start.line == 0 {
            if let Some(parent) = node.parent() {
                // XXX: This doesn't actually work.
                data.sourcepos.start.line = parent.data.borrow().sourcepos.start.line;
            }
        } else {
            data.sourcepos.start.line += preamble_lines;
        }
    }

    Ok(Some(InnerContext {
        body,
        source,
        body_source,
        preamble,
        origin,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        type DefaultLints<S> = HashMap<S, DefaultLint<S>>;
        let config: DefaultLints<&str> = default_lints_enum().collect();

        let serialized = toml::to_string_pretty(&config).unwrap();
        toml::from_str::<DefaultLints<String>>(&serialized).unwrap();
    }
}
