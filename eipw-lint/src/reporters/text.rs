/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::snippet::Snippet;

use std::cell::RefCell;
use std::fmt::{Debug, Write};

use super::{Error, Reporter};

#[derive(Debug, Default)]
pub struct Text<W> {
    inner: RefCell<W>,
}

impl<W> Reporter for Text<W>
where
    W: Write,
{
    fn report(&self, snippet: Snippet<'_>) -> Result<(), Error> {
        writeln!(self.inner.borrow_mut(), "{}", DisplayList::from(snippet)).map_err(Error::new)
    }
}

impl<W> Text<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner: inner.into(),
        }
    }

    pub fn into_inner(self) -> W {
        self.inner.into_inner()
    }
}
