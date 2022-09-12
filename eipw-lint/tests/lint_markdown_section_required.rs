/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::SectionRequired;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn one_missing() {
    let src = r#"---
header: value1
---

## Banana
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-section-req",
            SectionRequired(&["Banana", "Orange"]),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-section-req]: body is missing section(s): `Orange`
 |
 |
"#
    );
}

#[tokio::test]
async fn two_missing() {
    let src = r#"---
header: value1
---
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-section-req",
            SectionRequired(&["Banana", "Orange"]),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-section-req]: body is missing section(s): `Banana`, `Orange`
 |
 |
"#
    );
}

#[tokio::test]
async fn none_missing() {
    let src = r#"---
header: value1
---

## Banana

## Orange
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-section-req",
            SectionRequired(&["Banana", "Orange"]),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
