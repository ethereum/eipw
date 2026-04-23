/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::JsonSchema;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

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
5 | / ```hello
6 | | {
7 | | ```
  | |___^ EOF while parsing an object at line 2 column 0
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
5 | / ```hello
6 | | {"a": 3}
7 | | ```
  | |___^ 3 is not of type "string"
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
5 | / ```hello
6 | | {"a": "3"}
7 | | ```
  | |___^ "3" is not of type "integer"
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

// Regression test for https://github.com/ethereum/eipw/issues/100, where a
// panic would occur reporting a schema violation on a code block containing
// multi-byte UTF-8 characters.
#[tokio::test]
async fn non_ascii_code_block_does_not_panic() {
    let src = r#"---
header: value1
---

```hello
{"name": "Mazières"}
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
    "required": ["id"],
    "properties": {
        "id": { "type": "string" }
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
5 | / ```hello
6 | | {"name": "Mazières"}
7 | | ```
  | |___^ "id" is a required property
  |
  = help: see https://example.com/schema.json
"#
    );
}
