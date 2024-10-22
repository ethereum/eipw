/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Message};

use std::cell::RefCell;

use super::{Error, Reporter};

#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct Counts {
    pub error: usize,
    pub warning: usize,
    pub info: usize,
    pub note: usize,
    pub help: usize,
}

#[derive(Debug, Default)]
pub struct Count<T> {
    inner: T,
    counts: RefCell<Counts>,
}

impl<T> Reporter for Count<T>
where
    T: Reporter,
{
    fn report(&self, message: Message<'_>) -> Result<(), Error> {
        let mut counts = self.counts.borrow_mut();

        match message.level {
            Level::Error => counts.error += 1,
            Level::Warning => counts.warning += 1,
            Level::Info => counts.info += 1,
            Level::Note => counts.note += 1,
            Level::Help => counts.help += 1,
        }

        self.inner.report(message)
    }
}

impl<T> Count<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            counts: Default::default(),
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn counts(&self) -> Counts {
        *self.counts.borrow()
    }
}
