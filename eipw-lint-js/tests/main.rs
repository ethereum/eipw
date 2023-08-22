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
          "formatted": "error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          ^^^ has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
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
async fn lint_json_schema() {
    let mut path = PathBuf::from("tests");
    path.push("eips");
    path.push("eip-2000.md");

    let path = path.to_str().unwrap();

    let result = lint(vec![JsValue::from_str(path)], None)
        .await
        .ok()
        .unwrap();

    let actual: serde_json::Value = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = json! {
    [
       {
          "formatted": "error[markdown-json-cite]: code block of type `csl-json` does not conform to required schema\n  --> tests/eips/eip-2000.md:42:1\n   |\n42 | //     ```csl-json\n43 | ||     {\n44 | ||         \"type\": \"article\",\n45 | ||         \"id\": \"1\",\n46 |  |         \"URL\": \"3\"\n   | ||__________________^ \"3\" is not a \"uri\"\n   |  |__________________^ \"DOI\" is a required property\n   |\n   = help: see https://github.com/ethereum/eipw/blob/master/eipw-lint/src/lints/markdown/json_schema/citation.json\n   = help: see https://ethereum.github.io/eipw/markdown-json-cite/",
          "footer": [
             {
                "annotation_type": "Help",
                "id": null,
                "label": "see https://github.com/ethereum/eipw/blob/master/eipw-lint/src/lints/markdown/json_schema/citation.json"
             },
             {
                "annotation_type": "Help",
                "id": null,
                "label": "see https://ethereum.github.io/eipw/markdown-json-cite/"
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
                      "label": "\"3\" is not a \"uri\"",
                      "range": [
                         0,
                         86
                      ]
                   },
                   {
                      "annotation_type": "Error",
                      "label": "\"DOI\" is a required property",
                      "range": [
                         0,
                         86
                      ]
                   }
                ],
                "fold": false,
                "line_start": 42,
                "origin": "tests/eips/eip-2000.md",
                "source": "    ```csl-json\n    {\n        \"type\": \"article\",\n        \"id\": \"1\",\n        \"URL\": \"3\""
             }
          ],
          "title": {
             "annotation_type": "Error",
             "id": "markdown-json-cite",
             "label": "code block of type `csl-json` does not conform to required schema"
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
          "formatted": "warning[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          --- has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
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
                "line_start": 12,
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
async fn lint_one_with_default_lints() {
    let mut path = PathBuf::from("tests");
    path.push("eips");
    path.push("eip-1000.md");

    let path = path.to_str().unwrap();

    let opts = json!(
        {
            "default_lints": {
                "banana": {
                    "kind": "preamble-regex",
                    "name": "requires",
                    "mode": "includes",
                    "pattern": "banana",
                    "message": "requires must include banana"
                }
            }
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
          "formatted": "error[banana]: requires must include banana\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          ^^^ required pattern was not matched\n   |\n   = info: the pattern in question: `banana`\n   = help: see https://ethereum.github.io/eipw/banana/",
          "footer": [
             {
                "annotation_type": "Info",
                "id": null,
                "label": "the pattern in question: `banana`"
             },
             {
                "annotation_type": "Help",
                "id": null,
                "label": "see https://ethereum.github.io/eipw/banana/"
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
                      "label": "required pattern was not matched",
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
             "id": "banana",
             "label": "requires must include banana"
          }
       }
    ]
        };

    assert_eq!(expected, actual);
}

#[wasm_bindgen_test]
async fn lint_one_with_default_modifiers() {
    let mut path = PathBuf::from("tests");
    path.push("eips");
    path.push("eip-1000.md");

    let path = path.to_str().unwrap();

    let opts = json!(
        {
            "default_modifiers": [
                {
                    "kind": "set-default-annotation",
                    "name": "status",
                    "value": "Last Call",
                    "annotation_type": "info",
                }
            ]
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
          "formatted": "info[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          --- info: has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
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
                      "annotation_type": "Info",
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
             "annotation_type": "Info",
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
  --> tests/eips/eip-1000.md:12:10
   |
12 | requires: 20
   |          ^^^ has a less advanced status
   |
   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`
   = help: see https://ethereum.github.io/eipw/preamble-requires-status/"#;

    assert_eq!(expected, actual);
}
