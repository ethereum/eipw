/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use regex::Regex;

use snafu::{ensure, Backtrace, OptionExt, Snafu};

use std::collections::HashMap;

#[derive(Debug, Snafu)]
pub(crate) struct ParseErrors<'a> {
    backtrace: Backtrace,
    errors: Vec<Snippet<'a>>,
}

impl<'a> ParseErrors<'a> {
    pub fn into_errors(self) -> Vec<Snippet<'a>> {
        self.errors
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
#[non_exhaustive]
pub(crate) enum SplitError {
    #[snafu(context(suffix(false)))]
    LeadingGarbage,
    #[snafu(context(suffix(false)))]
    MissingStart,
    #[snafu(context(suffix(false)))]
    MissingEnd,
}

#[derive(Debug, Clone, Default)]
struct Fields<'a> {
    vec: Vec<Field<'a>>,
    map: HashMap<&'a str, usize>,
}

impl<'a> Fields<'a> {
    fn push(&mut self, field: Field<'a>) {
        let idx = self.vec.len();
        self.map.insert(field.name, idx);
        self.vec.push(field);
    }

    fn iter(&self) -> impl '_ + Iterator<Item = Field<'a>> {
        // Use the `Vec` to iterate, so lints can detect duplicates.
        self.vec.iter().copied()
    }

    fn by_name(&self, name: &str) -> Option<Field<'a>> {
        self.map.get(name).map(|idx| self.vec[*idx])
    }

    fn by_index(&self, index: usize) -> Option<Field<'a>> {
        self.vec.get(index).copied()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Preamble<'a> {
    fields: Fields<'a>,
}

impl<'a> Preamble<'a> {
    pub(crate) fn split(text: &'a str) -> Result<(&'a str, &'a str), SplitError> {
        let re_marker = Regex::new(r"(^|\n)---(\n|$)").unwrap();

        let mut iter = re_marker.find_iter(text);

        let start = iter.next().context(split_error::MissingStart)?;
        let end = iter.next().context(split_error::MissingEnd)?;

        ensure!(start.start() == 0, split_error::LeadingGarbage);

        let preamble = &text[start.end()..end.start()];
        let body = &text[end.end()..];

        Ok((preamble, body))
    }

    pub(crate) fn parse(origin: Option<&'a str>, text: &'a str) -> Result<Self, ParseErrors<'a>> {
        let lines = text.split('\n');
        let mut result: Result<Fields<'a>, Vec<Snippet<'a>>> = Ok(Default::default());

        for (index, line) in lines.enumerate() {
            let line_start = index + 1 + 1; // Lines start at one, plus `---\n`.

            result = match (result, Self::parse_line(origin, line_start, line)) {
                // Correct so far, and parsed a good name/value pair.
                (Ok(mut fields), Ok(new_field)) => {
                    fields.push(new_field);
                    Ok(fields)
                }

                // Had errors, and failed to parse a name/value pair.
                (Err(mut errors), Err(new_error)) => {
                    errors.push(new_error);
                    Err(errors)
                }

                // Was correct, but failed to parse the next name/value pair.
                (Ok(_), Err(new_error)) => Err(vec![new_error]),

                // Had errors, but successfully parsed a name/value pair.
                (r @ Err(_), Ok(_)) => r,
            };
        }

        match result {
            Ok(fields) => Ok(Self { fields }),
            Err(errors) => ParseErrorsSnafu { errors }.fail(),
        }
    }

    fn parse_line(
        origin: Option<&'a str>,
        line_start: usize,
        line: &'a str,
    ) -> Result<Field<'a>, Snippet<'a>> {
        let mut parts = line.splitn(2, ':');
        let name = parts.next().unwrap();
        let value = match parts.next() {
            Some(v) => v,
            None => {
                return Err(Snippet {
                    title: Some(Annotation {
                        label: Some("missing delimiter `:` in preamble field"),
                        id: None,
                        annotation_type: AnnotationType::Error,
                    }),
                    footer: vec![],
                    slices: vec![Slice {
                        source: line,
                        line_start,
                        origin,
                        annotations: vec![],
                        fold: false,
                    }],
                    opt: Default::default(),
                });
            }
        };

        Ok(Field {
            line_start,
            name,
            value,
            source: line,
        })
    }

    pub fn fields(&self) -> impl '_ + Iterator<Item = Field<'a>> {
        self.fields.iter()
    }

    pub fn by_name(&self, name: &str) -> Option<Field<'a>> {
        self.fields.by_name(name)
    }

    pub fn by_index(&self, index: usize) -> Option<Field<'a>> {
        self.fields.by_index(index)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Field<'a> {
    line_start: usize,
    source: &'a str,
    name: &'a str,
    value: &'a str,
}

impl<'a> Field<'a> {
    pub fn line_start(&self) -> usize {
        self.line_start
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn value(&self) -> &'a str {
        self.value
    }

    pub fn source(&self) -> &'a str {
        self.source
    }
}

#[cfg(test)]
mod tests {
    use annotate_snippets::display_list::DisplayList;
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn split_missing_start() {
        let input = "hello world\n";
        let actual = Preamble::split(input).unwrap_err();
        assert_matches!(actual, SplitError::MissingStart { .. });
    }

    #[test]
    fn split_missing_end() {
        let input = "---\nfoo: bar\n";
        let actual = Preamble::split(input).unwrap_err();
        assert_matches!(actual, SplitError::MissingEnd { .. });
    }

    #[test]
    fn split_leading_garbage() {
        let input = "hello world\n---\nfoo: bar\n---\n";
        let actual = Preamble::split(input).unwrap_err();
        assert_matches!(actual, SplitError::LeadingGarbage { .. });
    }

    #[test]
    fn split_line_feed() {
        let input = "---\nfoo: bar\n---\n\nhello world";
        let (preamble, body) = Preamble::split(input).unwrap();

        assert_eq!(preamble, "foo: bar");
        assert_eq!(body, "\nhello world");
    }

    #[test]
    fn split_carriage_return_then_line_feed() {
        let input = "---\r\nfoo: bar\r\n---\r\n\r\nhello world";
        let actual = Preamble::split(input).unwrap_err();
        assert_matches!(actual, SplitError::MissingStart { .. });
    }

    #[test]
    fn split_carriage_return() {
        let input = "---\rfoo: bar\r---\r\rhello world";
        let actual = Preamble::split(input).unwrap_err();
        assert_matches!(actual, SplitError::MissingStart { .. });
    }

    #[test]
    fn split_no_trailing_newline() {
        let input = "---\nfoo: bar\n---";
        let (preamble, body) = Preamble::split(input).unwrap();

        assert_eq!(preamble, "foo: bar");
        assert_eq!(body, "");
    }

    #[test]
    fn split() {
        let input = "---\nfoo: bar\n---\n\nhello world\n";
        let (preamble, body) = Preamble::split(input).unwrap();

        assert_eq!(preamble, "foo: bar");
        assert_eq!(body, "\nhello world\n");
    }

    #[test]
    fn parse_missing_colon() {
        let input = "foo: bar\nbanana split";
        let result = Preamble::parse(None, input).unwrap_err();
        assert_eq!(result.errors.len(), 1);

        let snippet = result.into_errors().pop().unwrap();
        let actual = DisplayList::from(snippet).to_string();
        let expected = r#"error: missing delimiter `:` in preamble field
  |
3 | banana split
  |"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse() {
        let input = "foo: bar\nbanana: split";
        let result = Preamble::parse(None, input).unwrap();
        let fields: Vec<_> = result.fields().collect();

        assert_matches!(
            fields.as_slice(),
            [
                Field {
                    line_start: 2,
                    name: "foo",
                    value: " bar",
                    source: "foo: bar",
                },
                Field {
                    line_start: 3,
                    name: "banana",
                    value: " split",
                    source: "banana: split",
                },
            ]
        );
    }
}
