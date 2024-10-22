/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::Snippet;

use crate::{
    lints::{Context, Error, Lint},
    LevelExt, SnippetExt,
};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneOf<S> {
    pub name: S,
    pub values: Vec<S>,
}

impl<S> Lint for OneOf<S>
where
    S: Debug + Display + AsRef<str> + for<'eq> PartialEq<&'eq str>,
{
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.name.as_ref()) {
            None => return Ok(()),
            Some(f) => f,
        };

        let value = field.value().trim();

        if self.values.iter().any(|e| e == &value) {
            return Ok(());
        }

        let label = format!("preamble header `{}` has an unrecognized value", self.name);

        let values: Vec<_> = self.values.iter().map(|a| a.as_ref()).collect();
        let slice_label = format!("must be one of: `{}`", values.join("`, `"));

        let name_count = field.name().len();
        let value_count = field.value().len();

        ctx.report(
            ctx.annotation_level().title(&label).id(slug).snippet(
                Snippet::source(field.source())
                    .fold(false)
                    .origin_opt(ctx.origin())
                    .line_start(field.line_start())
                    .annotation(
                        ctx.annotation_level()
                            .span_utf8(field.source(), name_count + 1, value_count)
                            .label(&slice_label),
                    ),
            ),
        )?;

        Ok(())
    }
}
