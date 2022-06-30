/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::RequiredIfEq;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn without_when_or_then() {
    let src = r#"---
header: value1
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "req-if-eq",
            RequiredIfEq {
                when: "when",
                equals: "equals",
                then: "then",
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn without_when_with_then() {
    let src = r#"---
header: value1
then: foo
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "req-if-eq",
            RequiredIfEq {
                when: "when",
                equals: "equals",
                then: "then",
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[req-if-eq]: preamble header `then` is only allowed when `when` is `equals`
  |
3 | then: foo
  | ^^^^^^^^^ defined here
  |
"#
    );
}

#[tokio::test]
async fn when_not_equal_with_then() {
    let src = r#"---
when: bar
header: value1
then: foo
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "req-if-eq",
            RequiredIfEq {
                when: "when",
                equals: "equals",
                then: "then",
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[req-if-eq]: preamble header `then` is only allowed when `when` is `equals`
  |
2 | when: bar
  | --------- info: unless equal to `equals`
  |
4 | then: foo
  | ^^^^^^^^^ remove this
  |
"#
    );
}

#[tokio::test]
async fn when_equal_with_then() {
    let src = r#"---
when: equals
header: value1
then: foo
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "req-if-eq",
            RequiredIfEq {
                when: "when",
                equals: "equals",
                then: "then",
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn when_equal_without_then() {
    let src = r#"---
when: equals
header: value1
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "req-if-eq",
            RequiredIfEq {
                when: "when",
                equals: "equals",
                then: "then",
            },
        )
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[req-if-eq]: preamble header `then` is required when `when` is `equals`
  |
2 | when: equals
  | ------------ info: defined here
  |
"#
    );
}
