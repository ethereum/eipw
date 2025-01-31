/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Length;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn unicode() {
    let src = r#"---
title: valué0
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-len-title",
            Length {
                name: "title",
                min: Some(10),
                max: None,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-len-title]: preamble header `title` value is too short (min 10)
  |
2 | title: valué0
  |       ^^^^^^^ too short
  |
"#,
    );
}

#[tokio::test]
async fn too_short() {
    let src = r#"---
title: value0
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-len-title",
            Length {
                name: "title",
                min: Some(10),
                max: None,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-len-title]: preamble header `title` value is too short (min 10)
  |
2 | title: value0
  |       ^^^^^^^ too short
  |
"#,
    );
}

#[tokio::test]
async fn too_long() {
    let src = r#"---
title: value0
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-len-title",
            Length {
                name: "title",
                max: Some(3),
                min: None,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-len-title]: preamble header `title` value is too long (max 3)
  |
2 | title: value0
  |       ^^^^^^^ too long
  |
"#,
    );
}

#[tokio::test]
async fn one() {
    let src = r#"---
title:v
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-len-title",
            Length {
                name: "title",
                max: Some(3),
                min: Some(2),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-len-title]: preamble header `title` value is too short (min 2)
  |
2 | title:v
  |       ^ too short
  |
"#,
    );
}
