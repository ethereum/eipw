/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::nodes::NodeValue;
use eipw_snippets::Level;

use crate::lints::{Error, Lint};

#[derive(Debug)]
pub struct HeadingsOnly;

impl Lint for HeadingsOnly {
    fn lint<'a>(&self, slug: &'a str, ctx: &crate::lints::Context<'a, '_>) -> Result<(), Error> {
        let annotation_type = Level::Error;
        let second = match ctx.body().descendants().nth(1) {
            Some(el) => el.data.borrow().to_owned().value,
            None => return ctx.report(annotation_type.title("Cannot submit an empty proposal")),
        };
        match second {
            NodeValue::Heading(_) => Ok(()),
            _ => {
                let annotation_type = Level::Error;
                ctx.report(annotation_type.title("Only Heading is allowed after FrontMatter"))
            }
        }
    }
}
