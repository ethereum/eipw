/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::regex::{Mode, Regex};
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn autolink_url_not_checked() {
    let src = r#"---
header: value1
---

A link <https://example.com/eip7002>.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "eip7002",
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
async fn regular_link_text_still_checked() {
    let src = r#"---
header: value1
---

A link [eip7002](https://example.com).
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "eip7002",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("boop"));
}

#[tokio::test]
async fn mixed_autolink_and_regular_link() {
    let src = r#"---
header: value1
---

Autolink <https://example.com/eip7002> and [regular](https://example.com).
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "eip7002",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should be empty because autolink URL is skipped and regular link text doesn't contain "eip7002"
    assert_eq!(reports, "");
}
