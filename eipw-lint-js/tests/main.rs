/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint_js::{format, lint};

use serde_json::json;

use std::path::PathBuf;

use wasm_bindgen::prelude::*;

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn lint_one() {
    let mut path = PathBuf::from("tests");
    path.push("eips");
    path.push("eip-1000.md");

    let path = path.to_str().unwrap();

    let result = lint(vec![JsValue::from_str(path)]).await.unwrap();

    let actual: serde_json::Value = result.into_serde().unwrap();
    let expected = json! {
    [
       {
          "footer": [
             {
                "annotation_type": "Help",
                "id": null,
                "label": "valid `status` values for this proposal are: `Draft`, `Stagnant`"
             }
          ],
          "opt": {
             "anonymized_line_numbers": false,
             "color": false
          },
          "slices": [
             {
                "annotations": [
                   {
                      "annotation_type": "Error",
                      "label": "has a less advanced status",
                      "range": [
                         9,
                         12
                      ]
                   }
                ],
                "fold": false,
                "line_start": 12,
                "origin": "tests/eips/eip-1000.md",
                "source": "requires: 20"
             }
          ],
          "title": {
             "annotation_type": "Error",
             "id": "preamble-requires-status",
             "label": "preamble header `requires` contains items not stable enough for a `status` of `Last Call`"
          }
       }
    ]
        };

    assert_eq!(expected, actual);
}

#[wasm_bindgen_test]
async fn format_one() {
    let mut path = PathBuf::from("tests");
    path.push("eips");
    path.push("eip-1000.md");

    let path = path.to_str().unwrap();

    let result = lint(vec![JsValue::from_str(path)]).await.unwrap();

    let snippets: Vec<serde_json::Value> = result.into_serde().unwrap();
    let snippet = JsValue::from_serde(&snippets[0]).unwrap();
    let actual = format(&snippet).unwrap();

    let expected = r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
  --> tests/eips/eip-1000.md:12:10
   |
12 | requires: 20
   |          ^^^ has a less advanced status
   |
   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`"#;

    assert_eq!(expected, actual);
}
