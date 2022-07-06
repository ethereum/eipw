/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::regex::{Mode, Regex};
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn exclude_present() {
    let src = r#"---
header: aa
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-regex",
            Regex {
                mode: Mode::Excludes,
                pattern: "aa",
                message: "bloop",
                name: "header",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-regex]: bloop
  |
2 | header: aa
  |        ^^^ prohibited pattern was matched
  |
  = info: the pattern in question: `aa`
"#,
    );
}

#[tokio::test]
async fn exclude_absent() {
    let src = r#"---
header: bb
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-regex",
            Regex {
                mode: Mode::Excludes,
                pattern: "aa",
                message: "bloop",
                name: "header",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn include_missing() {
    let src = r#"---
header: bb
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-regex",
            Regex {
                mode: Mode::Includes,
                pattern: "aa",
                message: "bloop",
                name: "header",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-regex]: bloop
  |
2 | header: bb
  |        ^^^ required pattern was not matched
  |
  = info: the pattern in question: `aa`
"#,
    );
}

#[tokio::test]
async fn include_present() {
    let src = r#"---
header: bb
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint(
            "preamble-regex",
            Regex {
                mode: Mode::Includes,
                pattern: "bb",
                message: "bloop",
                name: "header",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
