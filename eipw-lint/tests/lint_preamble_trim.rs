/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Trim;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn missing_space() {
    let src = r#"---
header:value0
header1:value0
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-trim", Trim)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-trim]: preamble header values must begin with a space
  |
2 | header:value0
  |        ^ space required here
  |
3 | header1:value0
  |         ^ space required here
  |
"#,
    );
}

#[tokio::test]
async fn extra_leading_space() {
    let src = r#"---
header:  value0
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-trim", Trim)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-trim]: preamble header `header` has extra whitespace
  |
2 | header:  value0
  |        ^^^^^^^^ value has extra whitespace
  |
"#,
    );
}

#[tokio::test]
async fn extra_trailing_space() {
    let src = r#"---
header: value0 
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-trim", Trim)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-trim]: preamble header `header` has extra whitespace
  |
2 | header: value0 
  |        ^^^^^^^^ value has extra whitespace
  |
"#,
    );
}
