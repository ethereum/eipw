/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::FutureDate;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn missing_date() {
    let src = r#"---
status: Last Call
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
async fn malformed_date() {
    let src = r#"---
last-call-deadline: not-a-date
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
async fn past_date() {
    let src = r#"---
last-call-deadline: 2000-01-01
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
        r#"error[preamble-future-date]: preamble header `last-call-deadline` must be today or a future date
  |
2 | last-call-deadline: 2000-01-01
  |                    ^^^^^^^^^^^ date is in the past
  |
"#,
    );
}

#[tokio::test]
async fn future_date() {
    let src = r#"---
last-call-deadline: 3000-12-12
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
