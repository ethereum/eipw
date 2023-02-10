/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Snippet};

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
    fn render(&self, snippet: &Snippet<'_>) -> Result<Option<String>, Error> {
        let id = match snippet.title.as_ref().and_then(|t| t.id) {
            Some(l) => l,
            None => return Ok(None),
        };

        let mut seen = self.seen.lock().unwrap();

        if !seen.insert(id.to_owned()) {
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
    fn report(&self, snippet: Snippet<'_>) -> Result<(), Error> {
        let rendered = self.render(&snippet)?;

        if let Some(ref message) = rendered {
            // This is a weird way to narrow the lifetime of `snippet`...
            let mut new_snippet = Snippet {
                title: snippet.title,
                footer: snippet.footer,
                slices: snippet.slices,
                opt: snippet.opt,
            };

            new_snippet.footer.push(Annotation {
                id: None,
                label: Some(message),
                annotation_type: AnnotationType::Help,
            });

            self.inner.report(new_snippet)
        } else {
            self.inner.report(snippet)
        }
    }
}
