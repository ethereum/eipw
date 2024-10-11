/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod null;
pub mod text;

use annotate_snippets::Message;

pub use self::null::Null;
pub use self::text::Text;

use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct Error {
    source: Box<dyn std::error::Error + 'static>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "report failed: {}", self.source)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

impl Error {
    pub fn new<S>(s: S) -> Self
    where
        S: std::error::Error + 'static,
    {
        Self {
            source: Box::new(s),
        }
    }
}

pub trait Reporter {
    fn report(&self, snippet: Message<'_>) -> Result<(), Error>;
}
