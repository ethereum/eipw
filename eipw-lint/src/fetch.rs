/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(feature = "tokio")]
pub mod tokio;

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

pub trait Fetch {
    fn fetch(&self, path: PathBuf)
        -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>>;
}

#[derive(Debug, Default)]
pub struct Null;

impl Fetch for Null {
    fn fetch(
        &self,
        _path: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        let fut = async { Err(std::io::ErrorKind::Unsupported.into()) };
        Box::pin(fut)
    }
}

#[cfg(feature = "tokio")]
pub use self::tokio::Tokio as DefaultFetch;

#[cfg(not(feature = "tokio"))]
pub use self::Null as DefaultFetch;
