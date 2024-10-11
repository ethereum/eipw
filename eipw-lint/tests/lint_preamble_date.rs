/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Date;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn single_digit_month() {
    let src = r#"---
header: 2022-1-01
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-date", Date("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-date]: preamble header `header` is not a date in the `YYYY-MM-DD` format
  |
2 | header: 2022-1-01
  |        ^^^^^^^^^^ invalid length
  |
"#,
    );
}

#[tokio::test]
async fn single_digit_day() {
    let src = r#"---
header: 2022-01-1
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-date", Date("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-date]: preamble header `header` is not a date in the `YYYY-MM-DD` format
  |
2 | header: 2022-01-1
  |        ^^^^^^^^^^ invalid length
  |
"#,
    );
}

#[tokio::test]
async fn invalid() {
    let src = r#"---
header: 12-13-2022
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-date", Date("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-date]: preamble header `header` is not a date in the `YYYY-MM-DD` format
  |
2 | header: 12-13-2022
  |        ^^^^^^^^^^^ input is out of range
  |
"#,
    );
}

#[tokio::test]
async fn invalid_unicode() {
    let src = r#"---
heáder: 12-13-2022
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-date", Date("heáder"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-date]: preamble header `heáder` is not a date in the `YYYY-MM-DD` format
  |
2 | heáder: 12-13-2022
  |        ^^^^^^^^^^^ input is out of range
  |
"#,
    );
}

#[tokio::test]
async fn valid() {
    let src = r#"---
header: 2022-01-02
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-date", Date("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
