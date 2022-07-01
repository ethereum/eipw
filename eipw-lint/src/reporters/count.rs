/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{AnnotationType, Snippet};

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
    pub other: usize,
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
    fn report(&self, snippet: Snippet<'_>) -> Result<(), Error> {
        let mut counts = self.counts.borrow_mut();

        match snippet.title.as_ref().map(|t| t.annotation_type) {
            Some(AnnotationType::Error) => counts.error += 1,
            Some(AnnotationType::Warning) => counts.warning += 1,
            Some(AnnotationType::Info) => counts.info += 1,
            Some(AnnotationType::Note) => counts.note += 1,
            Some(AnnotationType::Help) => counts.help += 1,
            None => counts.other += 1,
        }

        self.inner.report(snippet)
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
