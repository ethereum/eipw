/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::{Ast, NodeValue};

use crate::lints::{Error, Lint};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]
pub struct HeadingFirst;

impl Lint for HeadingFirst {
    fn lint<'a>(&self, slug: &'a str, ctx: &crate::lints::Context<'a, '_>) -> Result<(), Error> {
        let second = match ctx.body().descendants().nth(1) {
            Some(el) => el.data.borrow().to_owned(),
            None => return Ok(()),
        };

        let ast = match second {
            Ast {
                value: NodeValue::Heading(_),
                ..
            } => return Ok(()),
            other => other,
        };

        ctx.report(
            ctx.annotation_level()
                .title("Nothing is permitted between the preamble and the first heading")
                .id(slug)
                .snippet(ctx.ast_snippet(&ast, None, None)),
        )?;

        Ok(())
    }
}
