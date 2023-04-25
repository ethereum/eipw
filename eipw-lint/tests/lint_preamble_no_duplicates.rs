/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::no_duplicates::NoDuplicates;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn unicode() {
    let src = r#"---
header: válué0
other-header: value
header: válué1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-no-dup", NoDuplicates)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-no-dup]: preamble header `header` defined multiple times
  |
2 | header: válué0
  | -------------- info: first defined here
  |
4 | header: válué1
  | ^^^^^^^^^^^^^^ redefined here
  |
"#,
    );
}

#[tokio::test]
async fn one_duplicate() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-no-dup", NoDuplicates)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-no-dup]: preamble header `header` defined multiple times
  |
2 | header: value0
  | -------------- info: first defined here
  |
4 | header: value1
  | ^^^^^^^^^^^^^^ redefined here
  |
"#,
    );
}

#[tokio::test]
async fn two_duplicates() {
    let src = r#"---
header: value0
other-header: value
header: value1
other-header: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-no-dup", NoDuplicates)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-no-dup]: preamble header `header` defined multiple times
  |
2 | header: value0
  | -------------- info: first defined here
  |
4 | header: value1
  | ^^^^^^^^^^^^^^ redefined here
  |
error[preamble-no-dup]: preamble header `other-header` defined multiple times
  |
3 | other-header: value
  | ------------------- info: first defined here
  |
5 | other-header: bar
  | ^^^^^^^^^^^^^^^^^ redefined here
  |
"#,
    );
}

#[tokio::test]
async fn no_duplicates() {
    let src = r#"---
header: value0
other-header: value
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-no-dup", NoDuplicates)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
