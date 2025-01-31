/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::HtmlComments;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn warn() {
    let src = r#"---
header: value1
---
hello

<!-- multi-line
comment -->

text after
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-html-comments",
            HtmlComments {
                name: "header",
                warn_for: vec!["value1"],
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"warning[markdown-html-comments]: HTML comments are only allowed while `header` is one of: `value1`
  |
6 | <!-- multi-line
  |
"#
    );
}

#[tokio::test]
async fn error() {
    let src = r#"---
header: value2
---
hello

<!-- multi-line
comment -->

text after
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-html-comments",
            HtmlComments {
                name: "header",
                warn_for: vec!["value1"],
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-html-comments]: HTML comments are not allowed when `header` is `value2`
  |
6 | <!-- multi-line
  |
"#
    );
}
