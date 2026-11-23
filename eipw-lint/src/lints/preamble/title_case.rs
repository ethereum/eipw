/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Snippet};

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

const PUNCTUATION: &[char] = &[
    '(', ')', '[', ']', '{', '}', '"', '\'', ',', '.', ';', ':', '!', '?', '\u{201C}', '\u{201D}',
    '\u{2018}', '\u{2019}',
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct TitleCase<S> {
    pub name: S,
    pub small_words: Vec<S>,
    pub allowed: Vec<S>,
}

struct Word<'a> {
    start: usize,
    text: &'a str,
    code: bool,
}

fn words(value: &str) -> Vec<Word<'_>> {
    let mut backticks = 0;

    Regex::new(r"\S+")
        .unwrap()
        .find_iter(value)
        .filter_map(|token| {
            let raw = token.as_str();

            // A code span may contain whitespace, so every token between a pair of backticks
            // belongs to it.
            let code = backticks % 2 == 1 || raw.contains('`');
            backticks += raw.matches('`').count();

            let leading = raw.len() - raw.trim_start_matches(PUNCTUATION).len();
            let text = raw.trim_matches(PUNCTUATION);

            if text.is_empty() {
                return None;
            }

            Some(Word {
                start: token.start() + leading,
                text,
                code,
            })
        })
        .collect()
}

fn is_identifier(word: &str) -> bool {
    word.contains(|c: char| c == '_' || c == '/' || c.is_numeric())
        || word.chars().skip(1).any(char::is_uppercase)
}

impl<S> TitleCase<S>
where
    S: AsRef<str>,
{
    fn is_exempt(&self, word: &Word<'_>, is_edge: bool) -> bool {
        if word.code || is_identifier(word.text) {
            return true;
        }

        if self.allowed.iter().any(|a| a.as_ref() == word.text) {
            return true;
        }

        !is_edge && self.small_words.iter().any(|s| s.as_ref() == word.text)
    }
}

impl<S> Lint for TitleCase<S>
where
    S: Debug + Display + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(s) => s,
        };

        let words = words(field.value());
        let last = words.len().saturating_sub(1);
        let offset = field.name().len() + 1;

        let annotations: Vec<_> = words
            .iter()
            .enumerate()
            .filter(|(idx, word)| !self.is_exempt(word, *idx == 0 || *idx == last))
            .filter(|(_, word)| word.text.starts_with(char::is_lowercase))
            .map(|(_, word)| {
                ctx.annotation_level().span_utf8(
                    field.source(),
                    offset + word.start,
                    word.text.len(),
                )
            })
            .collect();

        if annotations.is_empty() {
            return Ok(());
        }

        let label = format!("preamble header `{}` should be in title case", self.name);

        let small_words: Vec<_> = self.small_words.iter().map(AsRef::as_ref).collect();
        let help = format!(
            "capitalize every word, except these when not first or last: {}",
            small_words.join(", "),
        );

        ctx.report(
            ctx.annotation_level()
                .title(&label)
                .id(slug)
                .snippet(
                    Snippet::source(field.source())
                        .fold(false)
                        .line_start(field.line_start())
                        .origin_opt(ctx.origin())
                        .annotations(annotations),
                )
                .footer(Level::Help.title(&help)),
        )?;

        Ok(())
    }
}
