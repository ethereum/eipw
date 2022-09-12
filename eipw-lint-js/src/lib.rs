/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::fetch::Fetch;
use eipw_lint::reporters::json::Json;
use eipw_lint::{default_lints, Linter};

use js_sys::{JsString, Object};

use serde::Deserialize;

use std::collections::HashMap;
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

#[derive(Debug, Deserialize)]
struct Opts {
    #[serde(default)]
    allow: Vec<String>,

    #[serde(default)]
    warn: Vec<String>,

    #[serde(default)]
    deny: Vec<String>,
}

impl Opts {
    fn apply<R>(self, mut linter: Linter<R>) -> Linter<R> {
        for allow in self.allow {
            linter = linter.allow(&allow);
        }

        if !self.warn.is_empty() {
            let mut lints: HashMap<_, _> = default_lints().collect();
            for warn in self.warn {
                let (k, v) = lints.remove_entry(warn.as_str()).unwrap();
                linter = linter.warn(k, v);
            }
        }

        if !self.deny.is_empty() {
            let mut lints: HashMap<_, _> = default_lints().collect();
            for deny in self.deny {
                let (k, v) = lints.remove_entry(deny.as_str()).unwrap();
                linter = linter.deny(k, v);
            }
        }

        linter
    }
}

#[wasm_bindgen]
pub async fn lint(sources: Vec<JsValue>, options: Option<Object>) -> Result<JsValue, JsError> {
    let sources: Vec<_> = sources
        .into_iter()
        .map(|v| v.as_string().unwrap())
        .map(PathBuf::from)
        .collect();

    let mut linter = Linter::new(Json::default()).set_fetch(NodeFetch);

    if let Some(options) = options {
        let opts: Opts = options.into_serde()?;
        linter = opts.apply(linter);
    }

    for source in &sources {
        linter = linter.check_file(source);
    }

    let reporter = linter.run().await?;

    Ok(JsValue::from_serde(&reporter.into_reports()).unwrap())
}

#[wasm_bindgen]
pub fn format(snippet: &JsValue) -> Result<String, JsError> {
    let value: serde_json::Value = snippet.into_serde()?;

    let obj = match value {
        serde_json::Value::Object(o) => o,
        _ => return Err(JsError::new("expected object")),
    };

    match obj.get("formatted") {
        Some(serde_json::Value::String(s)) => Ok(s.into()),
        _ => Err(JsError::new("expected `formatted` to be a string")),
    }
}
