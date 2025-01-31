/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::UintList;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn unicode() {
    let src = r#"---
header: 5, -1, 2, héllo world, 9
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-uint-list", UintList("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-uint-list]: preamble header `header` items must be unsigned integers
  |
2 | header: 5, -1, 2, héllo world, 9
  |           ^^^    ^^^^^^^^^^^^ not a non-negative integer
  |           |
  |           not a non-negative integer
  |
error[preamble-uint-list]: preamble header `header` items must be sorted in ascending order
  |
2 | header: 5, -1, 2, héllo world, 9
  |
"#,
    );
}

#[tokio::test]
async fn invalid() {
    let src = r#"---
header: 5, -1, 2, hello world, 9
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-uint-list", UintList("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-uint-list]: preamble header `header` items must be unsigned integers
  |
2 | header: 5, -1, 2, hello world, 9
  |           ^^^    ^^^^^^^^^^^^ not a non-negative integer
  |           |
  |           not a non-negative integer
  |
error[preamble-uint-list]: preamble header `header` items must be sorted in ascending order
  |
2 | header: 5, -1, 2, hello world, 9
  |
"#,
    );
}

#[tokio::test]
async fn valid() {
    let src = r#"---
header: 0, 2, 67, 100, 8888
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-uint-list", UintList("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
