/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::List;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn comma_first() {
    let src = r#"---
header: , example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header `header` cannot have empty items
  |
2 | header: , example.com/foo?bar
  |        ^ this item is empty
  |
"#,
    );
}

#[tokio::test]
async fn comma_last() {
    let src = r#"---
header: example.com/foo?bar,
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header `header` cannot have empty items
  |
2 | header: example.com/foo?bar,
  |                            ^ this item is empty
  |
"#,
    );
}

#[tokio::test]
async fn empty_middle() {
    let src = r#"---
header: foo,, example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header `header` cannot have empty items
  |
2 | header: foo,, example.com/foo?bar
  |            ^ this item is empty
  |
"#,
    );
}

#[tokio::test]
async fn missing_spaces() {
    let src = r#"---
header: foo,bar,example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header list items must begin with a space
  |
2 | header: foo,bar,example.com/foo?bar
  |            ^   ^ missing space
  |            |
  |            missing space
  |
"#,
    );
}

#[tokio::test]
async fn extra_spaces() {
    let src = r#"---
header: foo ,  bar,   bizz  , example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header list items have extra whitespace
  |
2 | header: foo ,  bar,   bizz  , example.com/foo?bar
  |         ^^^^ ^^^^^ ^^^^^^^^^ extra space
  |         |    |
  |         |    extra space
  |         extra space
  |
"#,
    );
}

#[tokio::test]
async fn empty() {
    let src = r#"---
header:
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn unicode() {
    let src = r#"---
author: Bánana Banana (@banana),  banana (@banana),  Orangé Banana (@banana)
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("author"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header list items have extra whitespace
  |
2 | author: Bánana Banana (@banana),  banana (@banana),  Orangé Banana (@banana)
  |                                 ^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^ extra space
  |                                 |
  |                                 extra space
  |
"#
    );
}

#[tokio::test]
async fn valid() {
    let src = r#"---
header: foo, bar, example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
