/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::NodeValue;

use crate::lints::{Error, Lint};

#[derive(Debug)]
pub struct HeadingFirst;

impl Lint for HeadingFirst {
    fn lint<'a>(&self, slug: &'a str, ctx: &crate::lints::Context<'a, '_>) -> Result<(), Error> {
        let second = match ctx.body().descendants().nth(1) {
            Some(el) => el.data.borrow().to_owned().value,
            None => {
                return ctx.report(
                    ctx.annotation_level()
                        .title("Cannot submit an empty proposal")
                        .id(slug),
                )
            }
        };
        match second {
            NodeValue::Heading(_) => Ok(()),
            _ => ctx.report(
                ctx.annotation_level()
                    .title("Nothing is permitted between the preamble and the first heading")
                    .id(slug),
            ),
        }
    }
}
