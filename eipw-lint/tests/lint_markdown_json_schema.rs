/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::JsonSchema;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn invalid_json() {
    let src = r#"---
header: value1
---

```hello
{
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![],
                schema: "{}",
                help: "see https://example.com/schema.json",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-json-schema]: code block of type `hello` does not contain valid JSON
  |
5 | ```hello
  | ^^^^^^^^ EOF while parsing an object at line 2 column 0
  |
"#
    );
}

#[tokio::test]
async fn empty_schema() {
    let src = r#"---
header: value1
---

```hello
{}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![],
                schema: "{}",
                help: "see https://example.com/schema.json",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn single_schema_valid() {
    let src = r#"---
header: value1
---

```hello
{"a": "b"}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![],
                help: "see https://example.com/schema.json",
                schema: r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Root",
    "type": "object",
    "required": ["a"],
    "properties": {
        "a": { "type": "string" }
    }
}"#,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn single_schema_invalid() {
    let src = r#"---
header: value1
---

```hello
{"a": 3}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![],
                help: "see https://example.com/schema.json",
                schema: r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Root",
    "type": "object",
    "required": ["a"],
    "properties": {
        "a": { "type": "string" }
    }
}"#,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-json-schema]: code block of type `hello` does not conform to required schema
  |
5 | ```hello
  | ^^^^^^^^ 3 is not of type "string"
  |
  = help: see https://example.com/schema.json
"#
    );
}

#[tokio::test]
async fn additional_schema_invalid() {
    let src = r#"---
header: value1
---

```hello
{"a": "3"}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![(
                    "http://example.com/additional.json",
                    r#"{ "type": "integer" }"#,
                )],
                help: "see https://example.com/schema.json",
                schema: r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Root",
    "type": "object",
    "required": ["a"],
    "properties": {
        "a": { "$ref": "http://example.com/additional.json" }
    }
}"#,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-json-schema]: code block of type `hello` does not conform to required schema
  |
5 | ```hello
  | ^^^^^^^^ "3" is not of type "integer"
  |
  = help: see https://example.com/schema.json
"#
    );
}

#[tokio::test]
async fn additional_schema_valid() {
    let src = r#"---
header: value1
---

```hello
{"a": 3}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![(
                    "http://example.com/additional.json",
                    r#"{ "type": "integer" }"#,
                )],
                help: "see https://example.com/schema.json",
                schema: r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Root",
    "type": "object",
    "required": ["a"],
    "properties": {
        "a": { "$ref": "http://example.com/additional.json" }
    }
}"#,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
