/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_snippets::{Level, Message};

use std::collections::HashSet;
use std::sync::Mutex;

use super::{Error, Reporter};

#[derive(Debug)]
pub struct AdditionalHelp<R, M> {
    inner: R,
    message: M,

    seen: Mutex<HashSet<String>>,
}

impl<R, M> AdditionalHelp<R, M> {
    pub fn new(inner: R, message: M) -> Self {
        Self {
            inner,
            message,
            seen: Default::default(),
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R, M> AdditionalHelp<R, M>
where
    M: Fn(&str) -> Result<String, Error>,
{
    fn render(&self, message: &Message<'_>) -> Result<Option<String>, Error> {
        let id = match message.id {
            Some(ref l) => l,
            None => return Ok(None),
        };

        let mut seen = self.seen.lock().unwrap();

        if !seen.insert(id.clone().into_owned()) {
            return Ok(None);
        }

        let rendered = (self.message)(id)?;

        Ok(Some(rendered))
    }
}

impl<R, M> Reporter for AdditionalHelp<R, M>
where
    R: Reporter,
    M: Fn(&str) -> Result<String, Error>,
{
    fn report(&self, message: Message<'_>) -> Result<(), Error> {
        let rendered = self.render(&message)?;

        if let Some(ref addition) = rendered {
            // I guess this shortens `message`'s lifetime?
            let mut message: Message<'_> = message;
            message.footer.push(Level::Help.title(addition));
            self.inner.report(message)
        } else {
            self.inner.report(message)
        }
    }
}
