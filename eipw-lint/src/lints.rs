/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod preamble;

use annotate_snippets::snippet::Snippet;

use comrak::nodes::AstNode;

use crate::preamble::Preamble;
use crate::reporters::{self, Reporter};

use snafu::Snafu;

use std::fmt::Debug;

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(context(false))]
    ReportFailed { source: reporters::Error },
}

#[derive(Debug)]
pub struct Context<'a> {
    pub(crate) preamble: Preamble<'a>,
    pub(crate) body_source: &'a str,
    pub(crate) body: &'a AstNode<'a>,
    pub(crate) origin: Option<&'a str>,
    pub(crate) reporter: &'a dyn Reporter,
}

impl<'a> Context<'a> {
    pub fn preamble(&self) -> &Preamble<'a> {
        &self.preamble
    }

    pub fn body_source(&self) -> &'a str {
        self.body_source
    }

    pub fn body(&self) -> &'a AstNode<'a> {
        self.body
    }

    pub fn origin(&self) -> Option<&'a str> {
        self.origin
    }

    pub fn report(&self, snippet: Snippet<'_>) -> Result<(), Error> {
        self.reporter.report(snippet)?;
        Ok(())
    }
}

pub trait Lint: Debug {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a>) -> Result<(), Error>;
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
