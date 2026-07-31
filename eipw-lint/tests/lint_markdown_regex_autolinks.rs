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
async fn excludes_autolink_text() {
    let src = r#"---
header: value1
---

<https://example.com/eip7002>
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

    // Autolink URL text should NOT be checked
    assert_eq!(reports, "");
}

#[tokio::test]
async fn excludes_regular_link_text() {
    let src = r#"---
header: value1
---

[eip7002](https://example.com/)
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

    // Regular link text SHOULD be checked
    assert_eq!(
        reports,
        r#"error[markdown-re]: boop
  |
5 | [eip7002](https://example.com/)
  |  ^^^^^^^
  |
  = info: the pattern in question: `eip7002`
"#
    );
}
