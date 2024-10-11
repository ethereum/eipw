/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::Message;

use super::{Error, Reporter};

#[derive(Debug)]
pub struct Null;

impl Reporter for Null {
    fn report(&self, _message: Message<'_>) -> Result<(), Error> {
        Ok(())
    }
}
