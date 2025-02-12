/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod config;
pub mod fetch;
pub mod lints;
pub mod modifiers;
pub mod reporters;
pub mod tree;

use config::Override;
use eipw_snippets::{Annotation, Level, Snippet};

use comrak::arena_tree::Node;
use comrak::nodes::Ast;
use comrak::Arena;
use formatx::formatx;
use lints::DefaultLint;
use modifiers::DefaultModifier;

use crate::config::Options;
use crate::lints::{Context, Error as LintError, FetchContext, InnerContext, Lint};
use crate::modifiers::Modifier;
use crate::reporters::Reporter;

use educe::Educe;

use eipw_preamble::{Preamble, SplitError};

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
    #[snafu(display("i/o error accessing `{}`", path.to_string_lossy()))]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    SliceFetched {
        lint: String,
        origin: Option<PathBuf>,
    },
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
pub struct LintSettings<'a> {
    _p: std::marker::PhantomData<&'a dyn Lint>,
    pub default_annotation_level: Level,
}

#[derive(Educe)]
#[educe(Debug)]
#[must_use]
pub struct Linter<'a, R> {
    lints: HashMap<String, (Option<Level>, Box<dyn Lint>)>,
    modifiers: Vec<Box<dyn Modifier>>,
    sources: Vec<Source<'a>>,

    proposal_format: String,

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
    pub fn with_options<M, L>(reporter: R, options: Options<M, L>) -> Self
    where
        L: 'static + Lint,
        M: 'static + Modifier,
    {
        let lints = options
            .lints
            .into_iter()
            .filter_map(|(slug, toggle)| Some((slug, (None, Box::new(toggle.into_lint()?) as _))))
            .collect();

        let proposal_format = options
            .fetch
            .map(|o| o.proposal_format)
            .unwrap_or_else(|| "eip-{}".into());

        Self {
            reporter,
            sources: Default::default(),
            fetch: Box::<fetch::DefaultFetch>::default(),
            modifiers: options
                .modifiers
                .into_iter()
                .map(|m| Box::new(m) as _)
                .collect(),
            lints,
            proposal_format,
        }
    }

    pub fn with_modifiers<I, M>(reporter: R, modifiers: I) -> Self
    where
        I: IntoIterator<Item = M>,
        M: 'static + Modifier,
    {
        let defaults =
            Options::<DefaultModifier<&'static str>, DefaultLint<&'static str>>::default();
        Self::with_options(
            reporter,
            Options {
                modifiers: modifiers.into_iter().collect(),
                lints: defaults.lints,
                fetch: defaults.fetch,
            },
        )
    }

    pub fn with_lints<I, S, L>(reporter: R, lints: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, L)>,
        L: 'static + Lint,
    {
        let defaults =
            Options::<DefaultModifier<&'static str>, DefaultLint<&'static str>>::default();
        Self::with_options(
            reporter,
            Options {
                modifiers: defaults.modifiers,
                lints: lints
                    .into_iter()
                    .map(|(s, l)| (s.into(), Override::enable(l)))
                    .collect(),
                fetch: Default::default(),
            },
        )
    }

    pub fn new(reporter: R) -> Self {
        Self::with_options::<DefaultModifier<&'static str>, DefaultLint<&'static str>>(
            reporter,
            Options::default(),
        )
    }

    pub fn warn<S, T>(self, slug: S, lint: T) -> Self
    where
        S: Into<String>,
        T: 'static + Lint,
    {
        self.add_lint(Some(Level::Warning), slug, lint)
    }

    pub fn deny<S, T>(self, slug: S, lint: T) -> Self
    where
        S: Into<String>,
        T: 'static + Lint,
    {
        self.add_lint(Some(Level::Error), slug, lint)
    }

    pub fn modify<T>(mut self, modifier: T) -> Self
    where
        T: 'static + Modifier,
    {
        self.modifiers.push(Box::new(modifier));
        self
    }

    fn add_lint<S, T>(mut self, level: Option<Level>, slug: S, lint: T) -> Self
    where
        S: Into<String>,
        T: 'static + Lint,
    {
        self.lints.insert(slug.into(), (level, Box::new(lint)));
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
                    fetch_proposals: Default::default(),
                };

                lint.1
                    .find_resources(&context)
                    .with_context(|_| LintSnafu {
                        origin: source_origin.clone(),
                    })?;

                let fetch_proposals = context.fetch_proposals.into_inner();

                // For now, string sources shouldn't be allowed to fetch external
                // resources. The origin field isn't guaranteed to be a file/URL,
                // and even if it was, we wouldn't know which of those to interpret
                // it as.
                ensure!(
                    fetch_proposals.is_empty() || !source.is_string(),
                    SliceFetchedSnafu {
                        lint: slug,
                        origin: source_origin.clone(),
                    }
                );

                if fetch_proposals.is_empty() {
                    continue;
                }

                let source_path = match source {
                    Source::File(p) => p,
                    _ => unreachable!(),
                };
                let source_dir = source_path.parent().unwrap_or_else(|| Path::new("."));
                let root = match source_path.file_name() {
                    Some(f) if f == "index.md" => source_dir.join(".."),
                    Some(_) | None => source_dir.to_path_buf(),
                };

                for proposal in fetch_proposals.into_iter() {
                    let entry = match fetched_eips.entry(proposal) {
                        hash_map::Entry::Occupied(_) => continue,
                        hash_map::Entry::Vacant(v) => v,
                    };
                    let basename =
                        formatx!(&self.proposal_format, proposal).expect("bad proposal format");

                    let mut plain_path = root.join(&basename);
                    plain_path.set_extension("md");
                    let plain = Source::File(&plain_path).fetch(&*self.fetch).await;

                    let mut index_path = root.join(&basename);
                    index_path.push("index.md");
                    let index = Source::File(&index_path).fetch(&*self.fetch).await;

                    let content = match (plain, index) {
                        (Ok(_), Ok(_)) => panic!(
                            "ambiguous proposal between `{}` and `{}`",
                            plain_path.to_string_lossy(),
                            index_path.to_string_lossy()
                        ),
                        (Ok(c), Err(_)) => Ok(c),
                        (Err(_), Ok(c)) => Ok(c),
                        (Err(e), Err(_)) => Err(e),
                    };

                    entry.insert(content);
                }
            }
        }

        let resources_arena = Arena::new();
        let mut parsed_eips = HashMap::new();

        for (number, result) in &fetched_eips {
            let source = match result {
                Ok(o) => o,
                Err(e) => {
                    parsed_eips.insert(*number, Err(e));
                    continue;
                }
            };

            let inner = match process(&self.reporter, &resources_arena, None, source)? {
                Some(s) => s,
                None => return Ok(self.reporter),
            };
            parsed_eips.insert(*number, Ok(inner));
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

            let mut settings = LintSettings {
                _p: std::marker::PhantomData,
                default_annotation_level: Level::Error,
            };

            for modifier in &self.modifiers {
                let context = Context {
                    inner: inner.clone(),
                    reporter: &self.reporter,
                    eips: &parsed_eips,
                    annotation_level: settings.default_annotation_level,
                };

                modifier.modify(&context, &mut settings)?;
            }

            for (slug, (annotation_level, lint)) in &lints {
                let annotation_level =
                    annotation_level.unwrap_or(settings.default_annotation_level);
                let context = Context {
                    inner: inner.clone(),
                    reporter: &self.reporter,
                    eips: &parsed_eips,
                    annotation_level,
                };

                lint.lint(slug, &context).with_context(|_| LintSnafu {
                    origin: origin.clone(),
                })?;
            }
        }

        Ok(self.reporter)
    }
}

fn comrak_options() -> comrak::Options<'static> {
    comrak::Options {
        extension: comrak::ExtensionOptions {
            table: true,
            autolink: true,
            footnotes: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn process<'a>(
    reporter: &dyn Reporter,
    arena: &'a Arena<Node<'a, RefCell<Ast>>>,
    origin: Option<&'a str>,
    source: &'a str,
) -> Result<Option<InnerContext<'a>>, Error> {
    let (preamble_source, body_source) = match Preamble::split(source) {
        Ok(v) => v,
        Err(SplitError::MissingStart { .. }) | Err(SplitError::LeadingGarbage { .. }) => {
            let mut footer = Vec::new();
            if source.as_bytes().get(3) == Some(&b'\r') {
                footer.push(Level::Help.title(
                    "found a carriage return (CR), use Unix-style line endings (LF) instead",
                ));
            }
            reporter
                .report(
                    Level::Error
                        .title("first line must be `---` exactly")
                        .snippet(
                            Snippet::source(source.lines().next().unwrap_or_default())
                                .origin_opt(origin)
                                .fold(false)
                                .line_start(1),
                        )
                        .footers(footer),
                )
                .map_err(LintError::from)
                .with_context(|_| LintSnafu {
                    origin: origin.map(PathBuf::from),
                })?;
            return Ok(None);
        }
        Err(SplitError::MissingEnd { .. }) => {
            reporter
                .report(
                    Level::Error
                        .title("preamble must be followed by a line containing `---` exactly"),
                )
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

    let options = comrak_options();

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

trait SnippetExt<'a> {
    fn origin_opt(self, origin: Option<&'a str>) -> Self;
}

impl<'a> SnippetExt<'a> for Snippet<'a> {
    fn origin_opt(self, origin: Option<&'a str>) -> Self {
        match origin {
            Some(origin) => self.origin(origin),
            None => self,
        }
    }
}

trait LevelExt {
    fn span_utf8(self, text: &str, start: usize, min_len: usize) -> Annotation;
}

impl LevelExt for Level {
    fn span_utf8(self, text: &str, start: usize, min_len: usize) -> Annotation {
        let end = ceil_char_boundary(text, start + min_len);
        self.span(start..end)
    }
}

/// Remove and replace with str::ceil_char_boundary if round_char_boundary stabilizes.
fn ceil_char_boundary(text: &str, index: usize) -> usize {
    if index > text.len() {
        return text.len();
    }

    for pos in index..=text.len() {
        if text.is_char_boundary(pos) {
            return pos;
        }
    }

    unreachable!();
}
