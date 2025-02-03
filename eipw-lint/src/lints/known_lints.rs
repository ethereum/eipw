/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{Context, FetchContext};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

use super::{markdown, preamble, Lint};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
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
    PreambleProposalRef(preamble::ProposalRef<S>),
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
    MarkdownNoBackticks {
        pattern: markdown::NoBackticks<S>,
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
    MarkdownHeadingsSpace(markdown::HeadingsSpace),
    MarkdownHeadingFirst(markdown::HeadingFirst),
}

impl<S> DefaultLint<S>
where
    S: Display + Debug + AsRef<str> + Clone + PartialEq<String> + for<'eq> PartialEq<&'eq str>,
{
    pub(crate) fn as_inner(&self) -> &dyn Lint {
        match self {
            Self::PreambleAuthor { name } => name,
            Self::PreambleDate { name } => name,
            Self::PreambleFileName(l) => l,
            Self::PreambleLength(l) => l,
            Self::PreambleList { name } => name,
            Self::PreambleNoDuplicates(l) => l,
            Self::PreambleOneOf(l) => l,
            Self::PreambleOrder { names } => names,
            Self::PreambleProposalRef(l) => l,
            Self::PreambleRegex(l) => l,
            Self::PreambleRequireReferenced(l) => l,
            Self::PreambleRequired { names } => names,
            Self::PreambleRequiredIfEq(l) => l,
            Self::PreambleRequiresStatus(l) => l,
            Self::PreambleTrim(l) => l,
            Self::PreambleUint { name } => name,
            Self::PreambleUintList { name } => name,
            Self::PreambleUrl { name } => name,

            Self::MarkdownHtmlComments(l) => l,
            Self::MarkdownJsonSchema(l) => l,
            Self::MarkdownLinkFirst { pattern } => pattern,
            Self::MarkdownNoBackticks { pattern } => pattern,
            Self::MarkdownLinkStatus(l) => l,
            Self::MarkdownProposalRef(l) => l,
            Self::MarkdownRegex(l) => l,
            Self::MarkdownRelativeLinks(l) => l,
            Self::MarkdownSectionOrder { sections } => sections,
            Self::MarkdownSectionRequired { sections } => sections,
            Self::MarkdownHeadingsSpace(l) => l,
            Self::MarkdownHeadingFirst(l) => l,
        }
    }
}

impl<S> DefaultLint<S>
where
    S: AsRef<str>,
{
    pub(crate) fn map_to_str(&self) -> DefaultLint<&str> {
        match self {
            Self::PreambleAuthor { name } => DefaultLint::PreambleAuthor {
                name: preamble::Author(name.0.as_ref()),
            },
            Self::PreambleDate { name } => DefaultLint::PreambleDate {
                name: preamble::Date(name.0.as_ref()),
            },
            Self::PreambleFileName(l) => DefaultLint::PreambleFileName(preamble::FileName {
                name: l.name.as_ref(),
                format: l.format.as_ref(),
            }),
            Self::PreambleLength(l) => DefaultLint::PreambleLength(preamble::Length {
                max: l.max,
                min: l.min,
                name: l.name.as_ref(),
            }),
            Self::PreambleList { name } => DefaultLint::PreambleList {
                name: preamble::List(name.0.as_ref()),
            },
            Self::PreambleNoDuplicates(_) => {
                DefaultLint::PreambleNoDuplicates(preamble::NoDuplicates)
            }
            Self::PreambleOneOf(l) => DefaultLint::PreambleOneOf(preamble::OneOf {
                name: l.name.as_ref(),
                values: l.values.iter().map(AsRef::as_ref).collect(),
            }),
            Self::PreambleOrder { names } => DefaultLint::PreambleOrder {
                names: preamble::Order(names.0.iter().map(AsRef::as_ref).collect()),
            },
            Self::PreambleProposalRef(l) => {
                DefaultLint::PreambleProposalRef(preamble::ProposalRef {
                    name: l.name.as_ref(),
                })
            }
            Self::PreambleRegex(l) => DefaultLint::PreambleRegex(preamble::Regex {
                message: l.message.as_ref(),
                mode: l.mode,
                name: l.name.as_ref(),
                pattern: l.pattern.as_ref(),
            }),
            Self::PreambleRequireReferenced(l) => {
                DefaultLint::PreambleRequireReferenced(preamble::RequireReferenced {
                    name: l.name.as_ref(),
                    requires: l.requires.as_ref(),
                })
            }
            Self::PreambleRequired { names } => DefaultLint::PreambleRequired {
                names: preamble::Required(names.0.iter().map(AsRef::as_ref).collect()),
            },
            Self::PreambleRequiredIfEq(l) => {
                DefaultLint::PreambleRequiredIfEq(preamble::RequiredIfEq {
                    equals: l.equals.as_ref(),
                    then: l.then.as_ref(),
                    when: l.when.as_ref(),
                })
            }
            Self::PreambleRequiresStatus(l) => {
                DefaultLint::PreambleRequiresStatus(preamble::RequiresStatus {
                    requires: l.requires.as_ref(),
                    status: l.status.as_ref(),
                    flow: l
                        .flow
                        .iter()
                        .map(|v| v.iter().map(AsRef::as_ref).collect())
                        .collect(),
                })
            }
            Self::PreambleTrim(_) => DefaultLint::PreambleTrim(preamble::Trim),
            Self::PreambleUint { name } => DefaultLint::PreambleUint {
                name: preamble::Uint(name.0.as_ref()),
            },
            Self::PreambleUintList { name } => DefaultLint::PreambleUintList {
                name: preamble::UintList(name.0.as_ref()),
            },
            Self::PreambleUrl { name } => DefaultLint::PreambleUrl {
                name: preamble::Url(name.0.as_ref()),
            },

            Self::MarkdownHtmlComments(l) => {
                DefaultLint::MarkdownHtmlComments(markdown::HtmlComments {
                    name: l.name.as_ref(),
                    warn_for: l.warn_for.iter().map(AsRef::as_ref).collect(),
                })
            }
            Self::MarkdownJsonSchema(l) => DefaultLint::MarkdownJsonSchema(markdown::JsonSchema {
                help: l.help.as_ref(),
                language: l.language.as_ref(),
                schema: l.schema.as_ref(),
                additional_schemas: l
                    .additional_schemas
                    .iter()
                    .map(|(a, b)| (a.as_ref(), b.as_ref()))
                    .collect(),
            }),
            Self::MarkdownLinkFirst { pattern } => DefaultLint::MarkdownLinkFirst {
                pattern: markdown::LinkFirst(pattern.0.as_ref()),
            },
            Self::MarkdownNoBackticks { pattern } => DefaultLint::MarkdownNoBackticks {
                pattern: markdown::NoBackticks(pattern.0.as_ref()),
            },
            Self::MarkdownLinkStatus(l) => DefaultLint::MarkdownLinkStatus(markdown::LinkStatus {
                pattern: l.pattern.as_ref(),
                status: l.status.as_ref(),
                flow: l
                    .flow
                    .iter()
                    .map(|v| v.iter().map(AsRef::as_ref).collect())
                    .collect(),
            }),
            Self::MarkdownProposalRef(_) => DefaultLint::MarkdownProposalRef(markdown::ProposalRef),
            Self::MarkdownRegex(l) => DefaultLint::MarkdownRegex(markdown::Regex {
                message: l.message.as_ref(),
                mode: l.mode,
                pattern: l.pattern.as_ref(),
            }),
            Self::MarkdownRelativeLinks(l) => {
                DefaultLint::MarkdownRelativeLinks(markdown::RelativeLinks {
                    exceptions: l.exceptions.iter().map(AsRef::as_ref).collect(),
                })
            }
            Self::MarkdownSectionOrder { sections } => DefaultLint::MarkdownSectionOrder {
                sections: markdown::SectionOrder(sections.0.iter().map(AsRef::as_ref).collect()),
            },
            Self::MarkdownSectionRequired { sections } => DefaultLint::MarkdownSectionRequired {
                sections: markdown::SectionRequired(sections.0.iter().map(AsRef::as_ref).collect()),
            },
            Self::MarkdownHeadingsSpace(l) => DefaultLint::MarkdownHeadingsSpace(l.clone()),
            Self::MarkdownHeadingFirst(l) => DefaultLint::MarkdownHeadingFirst(l.clone()),
        }
    }
}

impl<S> Lint for DefaultLint<S>
where
    S: std::fmt::Debug + AsRef<str>,
{
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), super::Error> {
        let lint = self.map_to_str();
        lint.as_inner().find_resources(ctx)
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), super::Error> {
        let lint = self.map_to_str();
        lint.as_inner().lint(slug, ctx)
    }
}

impl From<DefaultLint<&str>> for DefaultLint<String> {
    fn from(value: DefaultLint<&str>) -> Self {
        match value {
            DefaultLint::PreambleAuthor { name } => DefaultLint::PreambleAuthor {
                name: preamble::Author(name.0.to_string()),
            },
            DefaultLint::PreambleDate { name } => DefaultLint::PreambleDate {
                name: preamble::Date(name.0.to_string()),
            },
            DefaultLint::PreambleFileName(l) => DefaultLint::PreambleFileName(preamble::FileName {
                name: l.name.to_string(),
                format: l.format.to_string(),
            }),
            DefaultLint::PreambleLength(l) => DefaultLint::PreambleLength(preamble::Length {
                max: l.max,
                min: l.min,
                name: l.name.to_string(),
            }),
            DefaultLint::PreambleList { name } => DefaultLint::PreambleList {
                name: preamble::List(name.0.to_string()),
            },
            DefaultLint::PreambleNoDuplicates(_) => {
                DefaultLint::PreambleNoDuplicates(preamble::NoDuplicates)
            }
            DefaultLint::PreambleOneOf(l) => DefaultLint::PreambleOneOf(preamble::OneOf {
                name: l.name.to_string(),
                values: l.values.iter().map(|x| x.to_string()).collect(),
            }),
            DefaultLint::PreambleOrder { names } => DefaultLint::PreambleOrder {
                names: preamble::Order(names.0.iter().map(|x| x.to_string()).collect()),
            },
            DefaultLint::PreambleProposalRef(l) => {
                DefaultLint::PreambleProposalRef(preamble::ProposalRef {
                    name: l.name.to_string(),
                })
            }
            DefaultLint::PreambleRegex(l) => DefaultLint::PreambleRegex(preamble::Regex {
                message: l.message.to_string(),
                mode: l.mode,
                name: l.name.to_string(),
                pattern: l.pattern.to_string(),
            }),
            DefaultLint::PreambleRequireReferenced(l) => {
                DefaultLint::PreambleRequireReferenced(preamble::RequireReferenced {
                    name: l.name.to_string(),
                    requires: l.requires.to_string(),
                })
            }
            DefaultLint::PreambleRequired { names } => DefaultLint::PreambleRequired {
                names: preamble::Required(names.0.iter().map(|x| x.to_string()).collect()),
            },
            DefaultLint::PreambleRequiredIfEq(l) => {
                DefaultLint::PreambleRequiredIfEq(preamble::RequiredIfEq {
                    equals: l.equals.to_string(),
                    then: l.then.to_string(),
                    when: l.when.to_string(),
                })
            }
            DefaultLint::PreambleRequiresStatus(l) => {
                DefaultLint::PreambleRequiresStatus(preamble::RequiresStatus {
                    requires: l.requires.to_string(),
                    status: l.status.to_string(),
                    flow: l
                        .flow
                        .iter()
                        .map(|v| v.iter().map(|x| x.to_string()).collect())
                        .collect(),
                })
            }
            DefaultLint::PreambleTrim(_) => DefaultLint::PreambleTrim(preamble::Trim),
            DefaultLint::PreambleUint { name } => DefaultLint::PreambleUint {
                name: preamble::Uint(name.0.to_string()),
            },
            DefaultLint::PreambleUintList { name } => DefaultLint::PreambleUintList {
                name: preamble::UintList(name.0.to_string()),
            },
            DefaultLint::PreambleUrl { name } => DefaultLint::PreambleUrl {
                name: preamble::Url(name.0.to_string()),
            },

            DefaultLint::MarkdownHtmlComments(l) => {
                DefaultLint::MarkdownHtmlComments(markdown::HtmlComments {
                    name: l.name.to_string(),
                    warn_for: l.warn_for.iter().map(|x| x.to_string()).collect(),
                })
            }
            DefaultLint::MarkdownJsonSchema(l) => DefaultLint::MarkdownJsonSchema(markdown::JsonSchema {
                help: l.help.to_string(),
                language: l.language.to_string(),
                schema: l.schema.to_string(),
                additional_schemas: l
                    .additional_schemas
                    .iter()
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .collect(),
            }),
            DefaultLint::MarkdownLinkFirst { pattern } => DefaultLint::MarkdownLinkFirst {
                pattern: markdown::LinkFirst(pattern.0.to_string()),
            },
            DefaultLint::MarkdownNoBackticks { pattern } => DefaultLint::MarkdownNoBackticks {
                pattern: markdown::NoBackticks(pattern.0.to_string()),
            },
            DefaultLint::MarkdownLinkStatus(l) => DefaultLint::MarkdownLinkStatus(markdown::LinkStatus {
                pattern: l.pattern.to_string(),
                status: l.status.to_string(),
                flow: l
                    .flow
                    .iter()
                    .map(|v| v.iter().map(|x| x.to_string()).collect())
                    .collect(),
            }),
            DefaultLint::MarkdownProposalRef(_) => DefaultLint::MarkdownProposalRef(markdown::ProposalRef),
            DefaultLint::MarkdownRegex(l) => DefaultLint::MarkdownRegex(markdown::Regex {
                message: l.message.to_string(),
                mode: l.mode,
                pattern: l.pattern.to_string(),
            }),
            DefaultLint::MarkdownRelativeLinks(l) => {
                DefaultLint::MarkdownRelativeLinks(markdown::RelativeLinks {
                    exceptions: l.exceptions.iter().map(|x| x.to_string()).collect(),
                })
            }
            DefaultLint::MarkdownSectionOrder { sections } => DefaultLint::MarkdownSectionOrder {
                sections: markdown::SectionOrder(sections.0.iter().map(|x| x.to_string()).collect()),
            },
            DefaultLint::MarkdownSectionRequired { sections } => DefaultLint::MarkdownSectionRequired {
                sections: markdown::SectionRequired(sections.0.iter().map(|x| x.to_string()).collect()),
            },
            DefaultLint::MarkdownHeadingsSpace(l) => DefaultLint::MarkdownHeadingsSpace(l.clone()),
            DefaultLint::MarkdownHeadingFirst(l) => DefaultLint::MarkdownHeadingFirst(l.clone()),
        }
    }
}
