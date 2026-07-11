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
async fn excludes_link_match_in_text() {
    let src = r#"---
header: value1
---

[hi](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "hi",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-re]: boop
  |
5 | [hi](https://example.com/)
  |  ^^
  |
  = info: the pattern in question: `hi`
"#
    );
}

#[tokio::test]
async fn excludes_link_match_in_url() {
    let src = r#"---
header: value1
---

[hi](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "example",
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
async fn excludes_text() {
    let src = r#"---
header: value1
---

hi
hello
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "ello",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-re]: boop
  |
6 | hello
  |  ^^^^
  |
  = info: the pattern in question: `ello`
"#
    );
}

#[tokio::test]
async fn excludes_autolink() {
    let src = r#"---
header: value1
---

A link <https://example.com>.
Another link [Example](https://example.com).
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re",
            Regex {
                message: "boop",
                mode: Mode::Excludes,
                pattern: "example",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports, 
        ""
    );
}