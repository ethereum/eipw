/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::fetch::Fetch;
use eipw_lint::lints::{DefaultLint, Lint};
use eipw_lint::reporters::{AdditionalHelp, Json};
use eipw_lint::{default_lints, Linter};

use js_sys::{JsString, Object};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::ops::Deref;
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

    #[serde(default)]
    default_lints: Option<HashMap<String, DefaultLint<String>>>,
}

impl Opts {
    fn apply<'a, 'b: 'a, R>(&'a self, mut linter: Linter<'b, R>) -> Linter<'a, R> {
        for allow in &self.allow {
            linter = linter.allow(allow);
        }

        if !self.warn.is_empty() {
            let mut lints: HashMap<_, _> = default_lints().collect();
            for warn in &self.warn {
                let (k, v) = lints.remove_entry(warn.as_str()).unwrap();
                linter = linter.warn(k, v);
            }
        }

        if !self.deny.is_empty() {
            let mut lints: HashMap<_, _> = default_lints().collect();
            for deny in &self.deny {
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

    let reporter = Json::default();
    let reporter = AdditionalHelp::new(reporter, |t: &str| {
        Ok(format!("see https://ethereum.github.io/eipw/{}/", t))
    });

    let opts: Opts;
    let mut linter;
    if let Some(options) = options {
        opts = serde_wasm_bindgen::from_value(options.deref().clone())?;

        if let Some(ref lints) = opts.default_lints {
            linter = Linter::with_lints(
                reporter,
                lints
                    .iter()
                    .map(|(k, v)| (k.as_str(), Box::new(v.clone()) as Box<dyn Lint>)),
            );
        } else {
            linter = Linter::new(reporter);
        }

        linter = opts.apply(linter);
    } else {
        linter = Linter::new(reporter);
    }

    linter = linter.set_fetch(NodeFetch);

    for source in &sources {
        linter = linter.check_file(source);
    }

    let reporter = linter.run().await?;

    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    let js_value = reporter
        .into_inner()
        .into_reports()
        .serialize(&serializer)
        .unwrap();

    Ok(js_value)
}

#[wasm_bindgen]
pub fn format(snippet: &JsValue) -> Result<String, JsError> {
    let value: serde_json::Value = serde_wasm_bindgen::from_value(snippet.deref().clone())?;

    let obj = match value {
        serde_json::Value::Object(o) => o,
        _ => return Err(JsError::new("expected object")),
    };

    match obj.get("formatted") {
        Some(serde_json::Value::String(s)) => Ok(s.into()),
        _ => Err(JsError::new("expected `formatted` to be a string")),
    }
}
