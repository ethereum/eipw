/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::snippet::Snippet;

use eipw_lint::fetch::Fetch;
use eipw_lint::reporters::json::{snippet, Json};
use eipw_lint::Linter;

use js_sys::{JsString, JSON};

use std::fmt;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

#[wasm_bindgen(module = "node:fs/promises")]
extern "C" {
    #[wasm_bindgen(catch, js_name = readFile)]
    async fn read_file(path: &JsString, encoding: &JsString) -> Result<JsValue, JsValue>;
}

struct NodeFetch;

impl Fetch for NodeFetch {
    fn fetch(
        &self,
        path: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        let fut = async move {
            let path = match path.to_str() {
                Some(p) => JsString::from(p),
                None => return Err(std::io::ErrorKind::InvalidInput.into()),
            };

            let encoding = JsString::from("utf-8");

            match read_file(&path, &encoding).await {
                Ok(o) => Ok(o.as_string().unwrap()),
                Err(e) => {
                    let txt = format!("{:?}", e);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, Error(txt)))
                }
            }
        };

        Box::pin(fut)
    }
}

#[wasm_bindgen]
pub async fn lint(sources: Vec<JsValue>) -> Result<JsValue, JsValue> {
    let sources: Vec<_> = sources
        .into_iter()
        .map(|v| v.as_string().unwrap())
        .map(PathBuf::from)
        .collect();

    let mut linter = Linter::new(Json::default()).set_fetch(NodeFetch);

    for source in &sources {
        linter = linter.check_file(source);
    }

    let reporter = match linter.run().await {
        Ok(r) => r,
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };

    Ok(JsValue::from_serde(&reporter.into_reports()).unwrap())
}

#[wasm_bindgen]
pub fn format(snippet: &JsValue) -> Result<String, JsValue> {
    let json: String = JSON::stringify(snippet)?.into();
    let snippet: Snippet = serde_json::from_str::<snippet::SnippetDef>(&json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .into();

    Ok(format!("{}", DisplayList::from(snippet)))
}
