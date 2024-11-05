/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod known_lints;
pub mod markdown;
pub mod preamble;

use eipw_snippets::{Level, Message, Snippet};

use comrak::nodes::{Ast, AstNode, LineColumn};

use crate::reporters::{self, Reporter};
use crate::{LevelExt, SnippetExt};

use educe::Educe;

use eipw_preamble::Preamble;

pub use self::known_lints::DefaultLint;

use snafu::Snafu;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Deref;
use std::string::FromUtf8Error;

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(context(false))]
    ReportFailed { source: reporters::Error },
    #[snafu(context(false))]
    InvalidUtf8 { source: std::str::Utf8Error },
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

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::InvalidUtf8 {
            source: e.utf8_error(),
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
    pub(crate) eips: &'b HashMap<u32, Result<InnerContext<'b>, &'b crate::Error>>,
    #[educe(Debug(ignore))]
    pub(crate) reporter: &'b dyn Reporter,
    pub(crate) annotation_level: Level,
}

impl<'a, 'b> Context<'a, 'b>
where
    'b: 'a,
{
    pub fn preamble(&self) -> &Preamble<'a> {
        &self.inner.preamble
    }

    pub fn line_index(&self, line: usize) -> usize {
        let src = self.inner.source;
        let (idx, _) = src
            .bytes()
            .enumerate()
            .filter(|(_, chr)| *chr == b'\n')
            .take(line - 1)
            .last()
            .expect("could not find ast line in source");
        assert_eq!(src.as_bytes().get(idx), Some(&b'\n'));
        idx + 1
    }

    fn line_column_index(&self, line_column: LineColumn) -> usize {
        let line_index = self.line_index(line_column.line);
        line_index + line_column.column - 1
    }

    pub fn ast_source(&self, ast: &Ast) -> &'a str {
        let start = self.line_column_index(ast.sourcepos.start);
        let end = self.line_column_index(ast.sourcepos.end);
        &self.inner.source[start..=end]
    }

    pub fn ast_lines(&self, ast: &Ast) -> &'a str {
        let line_start_index = self.line_index(ast.sourcepos.start.line);
        let line_end_index = self.line_index(ast.sourcepos.end.line);
        let line_end_index = self.inner.source[line_end_index..]
            .find('\n')
            .map(|idx| idx + line_end_index)
            .unwrap_or_else(|| self.inner.source.len());

        &self.inner.source[line_start_index..line_end_index]
    }

    pub fn ast_snippet<'l, L: Into<Option<Level>>, O: Into<Option<&'l str>>>(
        &self,
        ast: &Ast,
        level: L,
        label: O,
    ) -> Snippet<'l>
    where
        'a: 'l,
    {
        let line_start_index = self.line_index(ast.sourcepos.start.line);
        let level = level.into().unwrap_or(self.annotation_level());

        let start_index = self.line_column_index(ast.sourcepos.start) - line_start_index;
        let end_index = self.line_column_index(ast.sourcepos.end) - line_start_index;

        let source = self.ast_lines(ast);
        let annotation = level.span_utf8(source, start_index, end_index + 1);

        let annotation = match label.into() {
            None => annotation,
            Some(label) => annotation.label(label.as_ref()),
        };

        Snippet::source(source)
            .fold(true)
            .line_start(ast.sourcepos.start.line)
            .origin_opt(self.origin())
            .annotation(annotation)
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

    pub fn annotation_level(&self) -> Level {
        self.annotation_level
    }

    pub fn report(&self, message: Message<'_>) -> Result<(), Error> {
        self.reporter.report(message)?;
        Ok(())
    }

    pub fn proposal(&self, proposal: u32) -> Result<Context<'b, 'b>, &crate::Error> {
        let inner = match self.eips.get(&proposal) {
            Some(Ok(i)) => i,
            Some(Err(e)) => return Err(e),
            None => panic!("no eip found for key `{}`", proposal),
        };

        Ok(Context {
            inner: inner.clone(),
            eips: self.eips,
            reporter: self.reporter,
            annotation_level: self.annotation_level,
        })
    }
}

#[derive(Debug)]
pub struct FetchContext<'a> {
    pub(crate) preamble: &'a Preamble<'a>,
    pub(crate) body: &'a AstNode<'a>,
    pub(crate) fetch_proposals: RefCell<HashSet<u32>>,
}

impl<'a> FetchContext<'a> {
    pub fn preamble(&self) -> &Preamble<'a> {
        self.preamble
    }

    pub fn body(&self) -> &'a AstNode<'a> {
        self.body
    }

    pub fn fetch_proposal(&self, proposal: u32) {
        self.fetch_proposals.borrow_mut().insert(proposal);
    }
}

pub trait Lint: Debug {
    fn find_resources(&self, _ctx: &FetchContext<'_>) -> Result<(), Error> {
        Ok(())
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error>;
}

impl Lint for Box<dyn Lint> {
    fn find_resources(&self, ctx: &FetchContext<'_>) -> Result<(), Error> {
        let lint: &dyn Lint = self.deref();
        lint.find_resources(ctx)
    }

    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let lint: &dyn Lint = self.deref();
        lint.lint(slug, ctx)
    }
}

#[cfg(test)]
mod tests {
    use comrak::{
        arena_tree::{Node, NodeEdge},
        nodes::NodeValue,
    };

    use super::*;

    fn get_context_ast_source(source: &str, pred: impl FnMut(&Ast) -> bool) -> String {
        let arena = comrak::Arena::new();
        let context = Context {
            annotation_level: Level::Error,
            eips: &Default::default(),
            reporter: &crate::reporters::Null,
            inner: crate::process(&crate::reporters::Null, &arena, Some("eip-1234.md"), source)
                .unwrap()
                .unwrap(),
        };

        let link = context
            .body()
            .traverse()
            .filter_map(|x| match x {
                NodeEdge::Start(Node { data, .. }) => Some(data.borrow().clone()),
                _ => None,
            })
            .filter(pred)
            .next()
            .unwrap();

        context.ast_source(&link).to_owned()
    }

    #[test]
    #[ignore] // https://github.com/kivikakk/comrak/issues/478
    fn context_ast_source_autolink_email() {
        let source = r#"
---
eip: 1234
---

foo@example.com hello world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Link(_)));
        assert_eq!(actual, "foo@example.com");
    }

    #[test]
    #[ignore] // https://github.com/kivikakk/comrak/issues/478
    fn context_ast_source_link_start() {
        let source = r#"
---
eip: 1234
---

<https://example.com> hello world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Link(_)));
        assert_eq!(actual, "<https://example.com>");
    }

    #[test]
    fn context_ast_source_inline_link_start() {
        let source = r#"
---
eip: 1234
---

[hello](https://example.com) hello world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Link(_)));
        assert_eq!(actual, "[hello](https://example.com)");
    }

    #[test]
    fn context_ast_source_emphasis_unicode() {
        let source = r#"
---
eip: 1234
---

*áemphá* hello world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Emph));
        assert_eq!(actual, "*áemphá*");
    }

    #[test]
    fn context_ast_source_emphasis_start() {
        let source = r#"
---
eip: 1234
---

*emphasis* hello world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Emph));
        assert_eq!(actual, "*emphasis*");
    }

    #[test]
    #[ignore] // https://github.com/kivikakk/comrak/issues/478
    fn context_ast_source_link_mid() {
        let source = r#"
---
eip: 1234
---

hello <https://example.com> world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Link(_)));
        assert_eq!(actual, "<https://example.com>");
    }

    #[test]
    fn context_ast_source_inline_link_mid() {
        let source = r#"
---
eip: 1234
---

hello [hello](https://example.com) world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Link(_)));
        assert_eq!(actual, "[hello](https://example.com)");
    }

    #[test]
    fn context_ast_source_emphasis_mid() {
        let source = r#"
---
eip: 1234
---

hello *emphasis* world
"#
        .trim();

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::Emph));
        assert_eq!(actual, "*emphasis*");
    }

    #[test]
    fn context_ast_source_code_block() {
        let source = r#"
---
eip: 1234
---

hello

```
this is a
multiline
code block
```

world
"#
        .trim();

        let expected = r#"```
this is a
multiline
code block
```"#;

        let actual = get_context_ast_source(source, |d| matches!(d.value, NodeValue::CodeBlock(_)));
        assert_eq!(actual, expected);
    }
}
