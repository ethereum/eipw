/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Snippet};

use comrak::{
    arena_tree::Node,
    nodes::{Ast, AstNode, NodeValue},
    Arena,
};
use html2text::render::TrivialDecorator;
use lru::LruCache;
use regex::{Regex, RegexSet};
use zspell::Dictionary;

use crate::{
    lints::{Context, Error, Lint},
    tree::{Next, TraverseExt, Visitor},
    SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Debug, Display},
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

const AFF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/dictionaries/dictionaries/en/index.aff"
));
const DICT: &str = concat!(
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/dictionaries/dictionaries/en/index.dic",
    )),
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/custom.dic"))
);

lazy_static::lazy_static! {
    static ref CACHE: Mutex<LruCache<String, Arc<Dictionary>>> =
        LruCache::new(NonZeroUsize::MIN).into();

    static ref ALLOW: RegexSet = RegexSet::new([
        "^(0x)?[[:xdigit:]]+$",
        "^[[:punct:]]+$",
    ]).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct Spell<S> {
    pub personal_dictionary: S,
}

impl<S> Lint for Spell<S>
where
    S: Display + Debug + AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let dict: Arc<Dictionary> = {
            CACHE
                .lock()
                .unwrap()
                .get_or_insert_ref(self.personal_dictionary.as_ref(), || {
                    zspell::builder()
                        .config_str(AFF)
                        .dict_str(DICT)
                        .personal_str(self.personal_dictionary.as_ref())
                        .build()
                        .expect("failed to build dictionary")
                        .into()
                })
                .clone()
        };

        let arena = Arena::new();
        let mut visitor = RemoveCode::new(&arena);
        ctx.body().traverse().visit(&mut visitor).unwrap();

        let mut output = Vec::new();
        let options = crate::comrak_options();

        comrak::format_html(visitor.current, &options, &mut output).unwrap();
        let text =
            html2text::from_read_with_decorator(output.as_slice(), usize::MAX, TrivialDecorator {})
                .unwrap();

        let mut mistakes: Vec<_> = dict
            .check_indices(&text)
            // Cannot use the index from `check_indices` because we spellcheck against the
            // plaintext rendered output, so the source position won't match.
            .map(|(_, w)| w)
            .filter(|w| !ALLOW.is_match(w))
            .collect::<HashSet<&str>>()
            .into_iter()
            .map(Mistake::new)
            .collect();

        let mut visitor = TextFind {
            slug,
            ctx,
            mistakes: &mut mistakes,
        };

        ctx.body().traverse().visit(&mut visitor)?;

        let bodyline = ctx.body().data.borrow().sourcepos.start.line;
        let source = ctx.line(bodyline);
        for Mistake {
            misspelling,
            reported,
            ..
        } in visitor.mistakes.iter_mut()
        {
            if *reported {
                continue;
            }

            let label = format!("the word `{misspelling}` is misspelled");
            ctx.report(
                ctx.annotation_level()
                    .title(&label)
                    .id(slug)
                    .snippet(
                        Snippet::source(source)
                            .line_start(bodyline)
                            .origin_opt(ctx.origin()),
                    )
                    .footer(Level::Warning.title("could not find a line number for this message")),
            )?;
        }

        Ok(())
    }
}

struct RemoveCode<'a> {
    arena: &'a Arena<AstNode<'a>>,
    current: &'a AstNode<'a>,
}

impl<'a> RemoveCode<'a> {
    fn new(arena: &'a Arena<AstNode<'a>>) -> Self {
        let current: &'a AstNode<'a> = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Document,
            (1, 1).into(),
        ))));
        Self { arena, current }
    }
}

impl<'a> Visitor for RemoveCode<'a> {
    type Error = std::convert::Infallible;

    fn enter(&mut self, node: &AstNode) -> Result<Next, Self::Error> {
        match node.data.borrow().value {
            NodeValue::Document => return Ok(Next::TraverseChildren),
            NodeValue::Code(_) => return Ok(Next::SkipChildren),
            NodeValue::CodeBlock(_) => return Ok(Next::SkipChildren),
            _ => (),
        };

        let node = AstNode::from(node.data.clone().into_inner());
        let node = self.arena.alloc(node);
        self.current.append(node);
        self.current = node;
        Ok(Next::TraverseChildren)
    }

    fn depart(&mut self, node: &AstNode) -> Result<(), Self::Error> {
        match node.data.borrow().value {
            NodeValue::Document => return Ok(()),
            NodeValue::Code(_) => return Ok(()),
            NodeValue::CodeBlock(_) => return Ok(()),
            _ => (),
        };

        self.current = self.current.parent().unwrap();
        Ok(())
    }
}

#[derive(Debug)]
struct Mistake<'a> {
    misspelling: &'a str,
    reported: bool,
    pattern: Regex,
}

impl<'a> Mistake<'a> {
    fn new(misspelling: &'a str) -> Self {
        let escaped = regex::escape(misspelling);
        let pattern = Regex::new(&format!(r"\b{escaped}\b")).unwrap();
        Self {
            misspelling,
            reported: false,
            pattern,
        }
    }
}

struct TextFind<'a, 'b, 'c> {
    slug: &'c str,
    mistakes: &'c mut [Mistake<'c>],
    ctx: &'c Context<'a, 'b>,
}

impl<'a, 'b, 'c> TextFind<'a, 'b, 'c> {
    fn check(
        &self,
        ast: &Ast,
        haystack: &str,
        mistake: &Mistake,
    ) -> Result<bool, <Self as Visitor>::Error> {
        let node_offset = match mistake.pattern.find(haystack) {
            Some(o) => o.start(),
            None => return Ok(false),
        };

        let source = self.ctx.source_for_text(ast.sourcepos.start.line, haystack);

        let line_offset = match source.find(haystack) {
            Some(l) => l,
            None => {
                // This would be really weird. Means the source we want to use doesn't contain the
                // text of the node we're checking. Instead of panicking, we'll just not mark this
                // mistake as reported and let the fallback handle it.
                return Ok(false);
            }
        };

        let start = node_offset + line_offset;
        let end = start + mistake.misspelling.len();

        let label = format!("the word `{}` is misspelled", mistake.misspelling);
        self.ctx.report(
            self.ctx
                .annotation_level()
                .title(&label)
                .id(self.slug)
                .snippet(
                    Snippet::source(&source)
                        .origin_opt(self.ctx.origin())
                        .line_start(ast.sourcepos.start.line)
                        .annotation(
                            self.ctx
                                .annotation_level()
                                .span(start..end)
                                .label("incorrectly spelled"),
                        ),
                ),
        )?;

        Ok(true)
    }
}

impl<'a, 'b, 'c> Visitor for TextFind<'a, 'b, 'c> {
    type Error = Error;

    fn enter_text(&mut self, ast: &Ast, txt: &str) -> Result<Next, Self::Error> {
        for ii in 0..self.mistakes.len() {
            if self.check(ast, txt, &self.mistakes[ii])? {
                self.mistakes[ii].reported = true;
            }
        }

        Ok(Next::TraverseChildren)
    }
}
