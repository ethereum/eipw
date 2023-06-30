/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

use super::{markdown, preamble, Lint};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum DefaultLint<S> {
    PreambleAuthor {
        name: preamble::Author<S>,
    },
    PreambleDate {
        name: preamble::Date<S>,
    },
    PreambleFileName(preamble::FileName<S>),
    PreambleLength(preamble::Length<S>),
    PreambleList {
        name: preamble::List<S>,
    },
    PreambleNoDuplicates(preamble::NoDuplicates),
    PreambleOneOf(preamble::OneOf<S>),
    PreambleOrder {
        names: preamble::Order<S>,
    },
    PreambleProposalRef {
        name: preamble::ProposalRef<S>,
    },
    PreambleRegex(preamble::Regex<S>),
    PreambleRequireReferenced(preamble::RequireReferenced<S>),
    PreambleRequired {
        names: preamble::Required<S>,
    },
    PreambleRequiredIfEq(preamble::RequiredIfEq<S>),
    PreambleRequiresStatus(preamble::RequiresStatus<S>),
    PreambleTrim(preamble::Trim),
    PreambleUint {
        name: preamble::Uint<S>,
    },
    PreambleUintList {
        name: preamble::UintList<S>,
    },
    PreambleUrl {
        name: preamble::Url<S>,
    },

    MarkdownHtmlComments(markdown::HtmlComments<S>),
    MarkdownJsonSchema(markdown::JsonSchema<S>),
    MarkdownLinkFirst {
        pattern: markdown::LinkFirst<S>,
    },
    MarkdownLinkStatus(markdown::LinkStatus<S>),
    MarkdownProposalRef(markdown::ProposalRef),
    MarkdownRegex(markdown::Regex<S>),
    MarkdownRelativeLinks(markdown::RelativeLinks<S>),
    MarkdownSectionOrder {
        sections: markdown::SectionOrder<S>,
    },
    MarkdownSectionRequired {
        sections: markdown::SectionRequired<S>,
    },
}

impl<S> DefaultLint<S>
where
    S: 'static
        + Display
        + Debug
        + AsRef<str>
        + Clone
        + PartialEq<String>
        + for<'eq> PartialEq<&'eq str>,
{
    pub(crate) fn boxed(self) -> Box<dyn Lint> {
        match self {
            Self::PreambleAuthor { name } => Box::new(name),
            Self::PreambleDate { name } => Box::new(name),
            Self::PreambleFileName(l) => Box::new(l),
            Self::PreambleLength(l) => Box::new(l),
            Self::PreambleList { name } => Box::new(name),
            Self::PreambleNoDuplicates(l) => Box::new(l),
            Self::PreambleOneOf(l) => Box::new(l),
            Self::PreambleOrder { names } => Box::new(names),
            Self::PreambleProposalRef { name } => Box::new(name),
            Self::PreambleRegex(l) => Box::new(l),
            Self::PreambleRequireReferenced(l) => Box::new(l),
            Self::PreambleRequired { names } => Box::new(names),
            Self::PreambleRequiredIfEq(l) => Box::new(l),
            Self::PreambleRequiresStatus(l) => Box::new(l),
            Self::PreambleTrim(l) => Box::new(l),
            Self::PreambleUint { name } => Box::new(name),
            Self::PreambleUintList { name } => Box::new(name),
            Self::PreambleUrl { name } => Box::new(name),

            Self::MarkdownHtmlComments(l) => Box::new(l),
            Self::MarkdownJsonSchema(l) => Box::new(l),
            Self::MarkdownLinkFirst { pattern } => Box::new(pattern),
            Self::MarkdownLinkStatus(l) => Box::new(l),
            Self::MarkdownProposalRef(l) => Box::new(l),
            Self::MarkdownRegex(l) => Box::new(l),
            Self::MarkdownRelativeLinks(l) => Box::new(l),
            Self::MarkdownSectionOrder { sections } => Box::new(sections),
            Self::MarkdownSectionRequired { sections } => Box::new(sections),
        }
    }
}
