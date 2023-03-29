/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint_js::{format, lint};

use js_sys::Object;

use serde::Serialize;

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

    let result = lint(vec![JsValue::from_str(path)], None)
        .await
        .ok()
        .unwrap();

    let actual: serde_json::Value = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = json! {
    [
       {
          "formatted": "error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:11:10\n   |\n11 | requires: 20\n   |          ^^^ has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
          "footer": [
             {
                "annotation_type": "Help",
                "id": null,
                "label": "valid `status` values for this proposal are: `Draft`, `Stagnant`"
             },
             {
                "annotation_type": "Help",
                "id": null,
                "label": "see https://ethereum.github.io/eipw/preamble-requires-status/"
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
                "line_start": 11,
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
async fn lint_one_with_options() {
    let mut path = PathBuf::from("tests");
    path.push("eips");
    path.push("eip-1000.md");

    let path = path.to_str().unwrap();

    let opts = json!(
       {
           "warn": ["preamble-requires-status"],
           "allow": [],
           "deny": []
       }
    );

    let opts_js = opts
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .unwrap();
    let opts = Object::try_from(&opts_js).unwrap().to_owned();

    let result = lint(vec![JsValue::from_str(path)], Some(opts))
        .await
        .ok()
        .unwrap();

    let actual: serde_json::Value = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = json! {
    [
       {
          "formatted": "warning[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:11:10\n   |\n11 | requires: 20\n   |          --- has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
          "footer": [
             {
                "annotation_type": "Help",
                "id": null,
                "label": "valid `status` values for this proposal are: `Draft`, `Stagnant`"
             },
             {
                "annotation_type": "Help",
                "id": null,
                "label": "see https://ethereum.github.io/eipw/preamble-requires-status/"
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
                      "annotation_type": "Warning",
                      "label": "has a less advanced status",
                      "range": [
                         9,
                         12
                      ]
                   }
                ],
                "fold": false,
                "line_start": 11,
                "origin": "tests/eips/eip-1000.md",
                "source": "requires: 20"
             }
          ],
          "title": {
             "annotation_type": "Warning",
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

    let result = lint(vec![JsValue::from_str(path)], None)
        .await
        .ok()
        .unwrap();

    let snippets: Vec<serde_json::Value> = serde_wasm_bindgen::from_value(result).unwrap();
    let snippet = snippets[0]
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .unwrap();
    let actual = format(&snippet).ok().unwrap();

    let expected = r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
  --> tests/eips/eip-1000.md:11:10
   |
11 | requires: 20
   |          ^^^ has a less advanced status
   |
   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`
   = help: see https://ethereum.github.io/eipw/preamble-requires-status/"#;

    assert_eq!(expected, actual);
}
