/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod markdown;
pub mod preamble;

use annotate_snippets::snippet::Snippet;

use comrak::nodes::AstNode;

use crate::preamble::Preamble;
use crate::reporters::{self, Reporter};

use educe::Educe;

use snafu::Snafu;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Deref;
use std::path::{Path, PathBuf};

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(context(false))]
    ReportFailed { source: reporters::Error },
    #[snafu(context(false))]
    InvalidUtf8 { source: std::string::FromUtf8Error },
    Custom {
        source: Box<dyn std::error::Error + 'static>,
    },
}

impl Error {
    pub fn custom<E>(source: E) -> Self
    where
        E: 'static + std::error::Error,
    {
        Self::Custom {
            source: Box::new(source) as Box<dyn std::error::Error>,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InnerContext<'a> {
    pub(crate) preamble: Preamble<'a>,
    pub(crate) source: &'a str,
    pub(crate) body_source: &'a str,
    pub(crate) body: &'a AstNode<'a>,
    pub(crate) origin: Option<&'a str>,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct Context<'a, 'b>
where
    'b: 'a,
{
    pub(crate) inner: InnerContext<'a>,
    pub(crate) eips: &'b HashMap<&'b Path, InnerContext<'b>>,
    #[educe(Debug(ignore))]
    pub(crate) reporter: &'b dyn Reporter,
}

impl<'a, 'b> Context<'a, 'b>
where
    'b: 'a,
{
    pub fn preamble(&self) -> &Preamble<'a> {
        &self.inner.preamble
    }

    /// XXX: comrak doesn't include a source field with its `AstNode`, so use
    ///      this instead. Don't expose it publicly since it's really hacky.
    ///      Yes, lines start at one.
    pub(crate) fn line(&self, mut line: u32) -> &'a str {
        assert_ne!(line, 0);
        line -= 1;
        self.inner
            .source
            .split('\n')
            .nth(line.try_into().unwrap())
            .unwrap()
    }

    pub fn body_source(&self) -> &'a str {
        self.inner.body_source
    }

    pub fn body(&self) -> &'a AstNode<'a> {
        self.inner.body
    }

    pub fn origin(&self) -> Option<&'a str> {
        self.inner.origin
    }

    pub fn report(&self, snippet: Snippet<'_>) -> Result<(), Error> {
        self.reporter.report(snippet)?;
        Ok(())
    }

    pub fn eip(&self, path: &Path) -> Context<'b, 'b> {
        let origin = self
            .origin()
            .expect("lint attempted to access an external resource without having an origin");

        let origin_path = PathBuf::from(origin);
        let root = origin_path.parent().unwrap_or_else(|| Path::new("."));

        let key = root.join(path);

        let inner = match self.eips.get(key.as_path()) {
            Some(i) => i,
            None => panic!("no eip found for key `{}`", key.display()),
        };

        Context {
            inner: inner.clone(),
            eips: self.eips,
            reporter: self.reporter,
        }
    }
}

#[derive(Debug)]
pub struct FetchContext<'a> {
    pub(crate) preamble: &'a Preamble<'a>,
    pub(crate) body: &'a AstNode<'a>,
    pub(crate) eips: RefCell<HashSet<PathBuf>>,
}

impl<'a> FetchContext<'a> {
    pub fn preamble(&self) -> &Preamble<'a> {
        self.preamble
    }

    pub fn body(&self) -> &'a AstNode<'a> {
        self.body
    }

    pub fn fetch(&self, path: PathBuf) {
        self.eips.borrow_mut().insert(path);
    }
}

pub trait Lint: Debug {
    fn find_resources<'a>(&self, _ctx: &FetchContext<'a>) -> Result<(), Error> {
        Ok(())
    }

    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error>;
}

impl Lint for Box<dyn Lint> {
    fn find_resources<'a>(&self, ctx: &FetchContext<'a>) -> Result<(), Error> {
        let lint: &dyn Lint = self.deref();
        lint.find_resources(ctx)
    }

    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let lint: &dyn Lint = self.deref();
        lint.lint(slug, ctx)
    }
}

pub(crate) trait LintExt: Lint {
    fn boxed(self) -> Box<dyn Lint>;
}

impl<T> LintExt for T
where
    T: 'static + Lint,
{
    fn boxed(self) -> Box<dyn Lint> {
        Box::new(self) as Box<dyn Lint>
    }
}
