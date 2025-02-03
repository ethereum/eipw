/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::lints::{DefaultLint, Lint};
use crate::modifiers::{self, DefaultModifier, Modifier};
use crate::Level;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

fn default_modifiers() -> Vec<DefaultModifier<&'static str>> {
    vec![
        DefaultModifier::SetDefaultAnnotation(modifiers::SetDefaultAnnotation {
            name: "status",
            value: "Stagnant",
            annotation_level: Level::Warning,
        }),
        DefaultModifier::SetDefaultAnnotation(modifiers::SetDefaultAnnotation {
            name: "status",
            value: "Withdrawn",
            annotation_level: Level::Warning,
        }),
    ]
}

fn default_lints() -> impl Iterator<Item = (&'static str, DefaultLint<&'static str>)> {
    use crate::lints::preamble::regex;
    use crate::lints::DefaultLint::*;
    use crate::lints::{markdown, preamble};

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
            PreambleProposalRef(preamble::ProposalRef {
                name: "title",
            }),
        ),
        (
            "preamble-refs-description",
            PreambleProposalRef(preamble::ProposalRef {
                name: "description",
            }),
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
                format: "eip-{}",
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
                pattern: markdown::LinkFirst(r"(?i)(?:eip|erc)-([0-9]+)"),
            }
        ),
        (
            "markdown-no-backticks",
            MarkdownNoBackticks {
                pattern: markdown::NoBackticks(r"(?i)(eip|erc)-[0-9]+"),
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
                pattern: r"(?i)(?:eip|erc)-([0-9]+).md$",
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
        (
            "markdown-headings-space",
            MarkdownHeadingsSpace(markdown::HeadingsSpace{}),
        ),
        (
            "markdown-heading-first",
            MarkdownHeadingFirst(markdown::HeadingFirst),
        )
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "bool", into = "bool")]
pub struct False;

#[derive(Debug, snafu::Snafu)]
pub struct FalseError;

impl TryFrom<bool> for False {
    type Error = FalseError;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if value {
            FalseSnafu.fail()
        } else {
            Ok(Self)
        }
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        false
    }
}

#[cfg(feature = "schema-version")]
impl schemars::JsonSchema for False {
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("False")
    }

    fn schema_name() -> String {
        Self::schema_id().into()
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Boolean.into()),
            format: None,
            const_value: Some(false.into()),
            ..Default::default()
        }
        .into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
#[serde(untagged, deny_unknown_fields)]
pub enum Override<T> {
    Disable { enabled: False },
    Enable(T),
}

impl<T> From<Override<T>> for Option<T> {
    fn from(value: Override<T>) -> Self {
        match value {
            Override::Enable(t) => Some(t),
            Override::Disable { .. } => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct FetchOptions {
    pub proposal_format: String,
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            proposal_format: "eip-{}".into(),
        }
    }
}

pub type DefaultOptions<S = String> = Options<DefaultModifier<S>, DefaultLint<S>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct Options<M, L> {
    #[serde(default = "Vec::<M>::new", skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<M>,

    #[serde(
        default = "HashMap::<String, Override<L>>::new",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub lints: HashMap<String, Override<L>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetch: Option<FetchOptions>,
}

#[cfg(feature = "schema-version")]
impl<M, L> Options<M, L>
where
    M: schemars::JsonSchema,
    L: schemars::JsonSchema,
{
    fn replace_floats(value: &mut serde_json::Value) {
        use serde_json::Value::*;

        let number = match value {
            Null | Bool(_) | String(_) => return,
            Number(n) => n,
            Array(a) => {
                for v in a.iter_mut() {
                    Self::replace_floats(v);
                }
                return;
            }
            Object(o) => {
                for (_, v) in o.iter_mut() {
                    Self::replace_floats(v);
                }
                return;
            }
        };

        if let Some(f) = number.as_f64() {
            *number = serde_json::Number::from(f as u64);
        }
    }

    pub fn schema_version() -> semver::Version {
        use olpc_cjson::CanonicalFormatter;
        use sha3::{Digest, Sha3_256};

        let schema = schemars::schema_for!(Self);
        let mut value = serde_json::to_value(&schema).unwrap();
        Self::replace_floats(&mut value);

        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, CanonicalFormatter::new());
        value.serialize(&mut ser).unwrap();

        let mut hasher = Sha3_256::new();
        hasher.update(&buf);
        let result = hasher.finalize();

        let mut pkg_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
        pkg_version.build = semver::BuildMetadata::new(&format!("{:x}", result)).unwrap();
        pkg_version
    }
}

impl<M, L> Default for Options<M, L>
where
    DefaultModifier<&'static str>: Into<M>,
    DefaultLint<&'static str>: Into<L>,
{
    fn default() -> Self {
        Self {
            modifiers: default_modifiers().into_iter().map(Into::into).collect(),
            lints: default_lints()
                .map(|(k, v)| (k.into(), Override::Enable(v.into())))
                .collect(),
            fetch: Some(FetchOptions::default()),
        }
    }
}

impl<M, L> Options<M, L>
where
    M: 'static + Modifier,
    L: 'static + Lint,
{
    pub fn to_iters(
        self,
    ) -> (
        impl Iterator<Item = Box<dyn Modifier>>,
        impl Iterator<Item = (String, Box<dyn Lint>)>,
        Option<FetchOptions>,
    ) {
        let modifiers = self
            .modifiers
            .into_iter()
            .map(|n| Box::new(n) as Box<dyn Modifier>);

        let lints = self
            .lints
            .into_iter()
            .filter_map(|(k, v)| match v {
                Override::Disable { .. } => None,
                Override::Enable(l) => Some((k, l)),
            })
            .map(|(k, v)| (k, Box::new(v) as Box<dyn Lint>));

        (modifiers, lints, self.fetch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{reporters, Linter};

    #[test]
    fn options_serialize_deserialize() {
        let options: Options<DefaultModifier<&str>, DefaultLint<&str>> = Options {
            fetch: Some(FetchOptions {
                proposal_format: "floop".into(),
            }),
            ..Default::default()
        };

        type StringOptions = Options<DefaultModifier<String>, DefaultLint<String>>;

        let serialized = toml::to_string_pretty(&options).unwrap();
        let actual = toml::from_str::<StringOptions>(&serialized).unwrap();

        #[allow(unused_must_use)]
        {
            Linter::with_options(reporters::Null, actual);
        }
    }
}
