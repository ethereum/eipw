/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::SectionOrder;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn one_extra() {
    let src = r#"---
header: value1
---

## Banana
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-section-order", SectionOrder(&[]))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-section-order]: body has extra section(s)
  |
5 | ## Banana
  |
"#
    );
}

#[tokio::test]
async fn two_extra() {
    let src = r#"---
header: value1
---

## Foo

## Banana

## Bar
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-section-order", SectionOrder(&["Banana"]))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-section-order]: body has extra section(s)
  |
5 | ## Foo
  |
9 | ## Bar
  |
"#
    );
}

#[tokio::test]
async fn out_of_order() {
    let src = r#"---
header: value1
---

## Banana

## Bar

## Foo
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-section-order",
            SectionOrder(&["Foo", "Banana", "Bar"]),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-section-order]: section `Banana` is out of order
  |
5 | ## Banana
  |
  = help: `Banana` should come after `Foo`
"#
    );
}

#[tokio::test]
async fn out_of_order_with_optional() {
    let src = r#"---
header: value1
---

## Banana

## Bar

## Foo
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-section-order",
            SectionOrder(&["Orange", "Foo", "Pear", "Banana", "Bar"]),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-section-order]: section `Banana` is out of order
  |
5 | ## Banana
  |
  = help: `Banana` should come after `Foo`
"#
    );
}

#[tokio::test]
async fn valid() {
    let src = r#"---
header: value1
---

## Foo

## Banana

## Bar
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-section-order",
            SectionOrder(&["Foo", "Banana", "Bar"]),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
