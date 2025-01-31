/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::FutureDate;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn past_date() {
    let src = r#"---
status: Last Call
last-call-deadline: 2023-12-12
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-future-date", FutureDate("last-call-deadline"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-future-date]: preamble header `last-call-deadline` must be today or a future date (today is 2024-12-12)
  |
3 | last-call-deadline: 2023-12-12
  |                     ^^^^^^^^^^ must be today or a future date
  |
"#,
    );
}

#[tokio::test]
async fn today_date() {
    let src = r#"---
status: Last Call
last-call-deadline: 2024-12-12
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-future-date", FutureDate("last-call-deadline"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Today's date should be valid
    assert_eq!(reports, "");
}

#[tokio::test]
async fn future_date() {
    let src = r#"---
status: Last Call
last-call-deadline: 2025-12-12
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-future-date", FutureDate("last-call-deadline"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn not_last_call() {
    let src = r#"---
status: Draft
last-call-deadline: 2023-12-12
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-future-date", FutureDate("last-call-deadline"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should not error when status is not Last Call, even with past date
    assert_eq!(reports, "");
}
