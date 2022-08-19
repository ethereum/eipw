/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod snippet;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::snippet::Snippet;

use self::snippet::SnippetDef;

use serde::Serialize;

use serde_json::Value;

use std::cell::RefCell;

use super::{Error, Reporter};

#[derive(Debug, Serialize, Default)]
#[serde(transparent)]
pub struct Json {
    reports: RefCell<Vec<Value>>,
}

impl Reporter for Json {
    fn report(&self, snippet: Snippet<'_>) -> Result<(), Error> {
        let def = SnippetDef::from(snippet);

        let mut value = serde_json::to_value(&def).map_err(Error::new)?;
        let obj = value.as_object_mut().unwrap();

        // Because `SnippetDef` borrows while deserializing, it breaks with
        // escaped characters, so we pre-format the errors here.
        let snippet = Snippet::from(def);
        let formatted = format!("{}", DisplayList::from(snippet));
        obj.insert("formatted".into(), Value::String(formatted));

        self.reports.borrow_mut().push(value);
        Ok(())
    }
}

impl Json {
    pub fn into_reports(self) -> Vec<Value> {
        self.reports.into_inner()
    }
}
