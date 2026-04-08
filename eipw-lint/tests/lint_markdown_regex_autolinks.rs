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

#[tokio::test]
async fn excludes_bare_autolink_text() {
    let src = r#"---
header: value1
---

See https://example.com/eip7002 for details.
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
async fn excludes_www_autolink_text() {
    let src = r#"---
header: value1
---

See www.example.com/eip7002 for details.
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
async fn excludes_email_autolink_text() {
    let src = r#"---
header: value1
---

<eip7002@example.com>
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
async fn excludes_url_nested_in_link_text() {
    let src = r#"---
header: value1
---

[a link to **https://example.com/eip7002** I guess](https://example.com/eip7002)
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

    assert_eq!(
        reports,
        r#"error[markdown-re]: boop
  |
5 | [a link to **https://example.com/eip7002** I guess](https://example.com/eip7002)
  |                                  ^^^^^^^
  |
  = info: the pattern in question: `eip7002`
"#
    );
}

#[tokio::test]
async fn excludes_url_beside_other_link_text() {
    let src = r#"---
header: value1
---

[https://example.com/eip7002**!**](https://example.com/eip7002)
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

    assert_eq!(
        reports,
        r#"error[markdown-re]: boop
  |
5 | [https://example.com/eip7002**!**](https://example.com/eip7002)
  |                      ^^^^^^^
  |
  = info: the pattern in question: `eip7002`
"#
    );
}

#[tokio::test]
async fn excludes_link_title() {
    let src = r#"---
header: value1
---

[https://example.com/](https://example.com/ "eip7002")
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

    assert_eq!(
        reports,
        r#"error[markdown-re]: boop
  |
5 | [https://example.com/](https://example.com/ "eip7002")
  |                                              ^^^^^^^
  |
  = info: the pattern in question: `eip7002`
"#
    );
}
