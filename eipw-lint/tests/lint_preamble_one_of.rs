/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::OneOf;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn unrecognized_value() {
    let src = r#"---
a1: value
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-one-of",
            OneOf {
                name: "a1",
                values: &["v1", "v2"],
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-one-of]: preamble header `a1` has an unrecognized value
  |
2 | a1: value
  |    ^^^^^^ must be one of: `v1`, `v2`
  |
"#
    );
}

#[tokio::test]
async fn different_unrecognized_value() {
    let src = r#"---
a1:valuesaonehuntsoaehustnaoehu
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-one-of",
            OneOf {
                name: "a1",
                values: &["v1", "v2"],
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-one-of]: preamble header `a1` has an unrecognized value
  |
2 | a1:valuesaonehuntsoaehustnaoehu
  |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ must be one of: `v1`, `v2`
  |
"#
    );
}

#[tokio::test]
async fn recognized_value() {
    let src = r#"---
header: value1
a1: v2
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-one-of",
            OneOf {
                name: "a1",
                values: &["v1", "v2"],
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn no_header() {
    let src = r#"---
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-one-of",
            OneOf {
                name: "a1",
                values: &["v1", "v2"],
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
