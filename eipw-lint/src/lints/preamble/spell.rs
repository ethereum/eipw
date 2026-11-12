/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use crate::{
    lints::{
        spell::{dictionary, is_allowed},
        Context, Error, Lint,
    },
    SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct Spell<S> {
    pub names: Vec<S>,
    pub personal_dictionary: S,
}

impl<S> Lint for Spell<S>
where
    S: Display + Debug + AsRef<str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let dict = dictionary(self.personal_dictionary.as_ref());

        for name in &self.names {
            let field = match ctx.preamble().by_name(name.as_ref()) {
                Some(field) => field,
                None => continue,
            };

            for (offset, misspelling) in dict
                .check_indices(field.value())
                .filter(|(_, word)| !is_allowed(word))
            {
                let source = field.source();
                let start = field.name().len() + 1 + offset;
                let end = start + misspelling.len();
                let title = format!("the word `{misspelling}` is misspelled");

                ctx.report(
                    ctx.annotation_level().title(&title).id(slug).snippet(
                        Snippet::source(source)
                            .fold(false)
                            .origin_opt(ctx.origin())
                            .line_start(field.line_start())
                            .annotation(
                                ctx.annotation_level()
                                    .span(start..end)
                                    .label("incorrectly spelled"),
                            ),
                    ),
                )?;
            }
        }

        Ok(())
    }
}
