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
async fn non_ascii_invalid() {
    let src = "---\nheader: value1\n---\n\n日本語\n\n```hello\n\"not an integer\"\n```\n";

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![],
                schema: r#"{ "type": "integer" }"#,
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
        r#"error[markdown-json-schema]: code block of type `hello` does not conform to required schema
  |
7 | / ```hello
8 | | "not an integer"
9 | | ```
  | |___^ "not an integer" is not of type "integer"
  |
  = help: see https://example.com/schema.json
"#
    );
}

#[tokio::test]
async fn non_ascii_valid() {
    let src = "---\nheader: value1\n---\n\n日本語\n\n```hello\n42\n```\n";

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-json-schema",
            JsonSchema {
                language: "hello",
                additional_schemas: vec![],
                schema: r#"{ "type": "integer" }"#,
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
