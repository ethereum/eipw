/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::AllowedOnlyIfEq;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

async fn reports_for(src: &str) -> String {
    Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "allowed-only-if-eq",
            AllowedOnlyIfEq {
                when: "mode",
                equals: "advanced",
                then: "advanced-setting",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner()
}

#[tokio::test]
async fn without_when_or_then() {
    let src = r#"---
header: value1
---
hello world"#;

    assert_eq!(reports_for(src).await, "");
}

#[tokio::test]
async fn when_equal_without_then() {
    let src = r#"---
mode: advanced
header: value1
---
hello world"#;

    assert_eq!(reports_for(src).await, "");
}

#[tokio::test]
async fn when_equal_with_then() {
    let src = r#"---
mode: advanced
advanced-setting: enabled
---
hello world"#;

    assert_eq!(reports_for(src).await, "");
}

#[tokio::test]
async fn when_not_equal_without_then() {
    let src = r#"---
mode: basic
header: value1
---
hello world"#;

    assert_eq!(reports_for(src).await, "");
}

#[tokio::test]
async fn without_when_with_then() {
    let src = r#"---
header: value1
advanced-setting: enabled
---
hello world"#;

    assert_eq!(
        reports_for(src).await,
        r#"error[allowed-only-if-eq]: preamble header `advanced-setting` is only allowed when `mode` is `advanced`
  |
3 | advanced-setting: enabled
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ defined here
  |
"#
    );
}

#[tokio::test]
async fn when_not_equal_with_then() {
    let src = r#"---
mode: basic
header: value1
advanced-setting: enabled
---
hello world"#;

    assert_eq!(
        reports_for(src).await,
        r#"error[allowed-only-if-eq]: preamble header `advanced-setting` is only allowed when `mode` is `advanced`
  |
2 | mode: basic
  | ----------- info: unless equal to `advanced`
  |
4 | advanced-setting: enabled
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ remove this
  |
"#
    );
}
