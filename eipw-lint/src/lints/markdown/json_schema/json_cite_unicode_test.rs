/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::lints::markdown::JsonSchema;
use crate::reporters::Text;
use crate::Linter;

#[tokio::test]
async fn json_cite_unicode_panic() {
    // This reproduces issue #100: panic with non-ASCII input in markdown-json-cite
    // The schema requires "title" as a string, but the JSON only has "family".
    // This causes schema validation to fail, which triggers the annotation span code.
    let src = r#"---
eip: 3
---

Some text somewhere that needs a citation. [^1]

[^1]:
    ```csl-json
    {
      "author": [
        {
          "family": "Mazières"
        }
      ]
    }
    ```
"#;

    let schema = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["title"],
    "properties": {
        "title": { "type": "string" }
    }
}"#;

    let result = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-cite",
            JsonSchema {
                language: "csl-json",
                additional_schemas: vec![],
                schema,
                help: "see https://example.com/schema.json",
            },
        )
        .check_slice(None, src)
        .run()
        .await;

    match result {
        Ok(reports) => {
            println!("OK:\n{}", reports.into_inner());
        }
        Err(e) => {
            println!("Err: {:?}", e);
        }
    }
}
