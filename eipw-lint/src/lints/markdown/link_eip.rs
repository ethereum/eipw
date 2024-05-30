/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// use annotate_snippets::snippet::{Annotation, Slice, Snippet};

// use comrak::nodes::{Ast, AstNode, NodeCode, NodeCodeBlock, NodeHtmlBlock};
 
use crate::lints::{Context, Error, Lint}; // FetchContext
// use crate::tree::{self, Next, TraverseExt};
 
// use regex::Regex;
 
use serde::{Deserialize, Serialize};
 
// use std::collections::HashSet;
use std::fmt::{Debug, Display};
// use std::path::PathBuf;
 
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LinkEip<S>(pub S);
 
impl<S> Lint for LinkEip<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        Ok(())
    }
}