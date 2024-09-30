/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::LinkFirst;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn unlinked_then_linked_with_header() {
    let src = r#"---
eip: 4444
---
eip-1234

[eip-1234](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-first",
            LinkFirst(r"(?i)(?:eip|erc)-([0-9]+)"),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-link-first]: the first match of the given pattern must be a link
  |
4 | eip-1234
  |
  = info: the pattern in question: `(?i)(?:eip|erc)-([0-9]+)`
"#
    );
}

#[tokio::test]
async fn unlinked_then_linked() {
    let src = r#"---
header: value1
---
hello

[ello](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-first", LinkFirst("ello"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-link-first]: the first match of the given pattern must be a link
  |
4 | hello
  |
  = info: the pattern in question: `ello`
"#
    );
}

#[tokio::test]
async fn linked_then_unlinked() {
    let src = r#"---
header: value1
---
[ello](https://example.com/)

hello
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-first", LinkFirst("ello"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn img_then_linked() {
    let src = r#"---
header: value1
---
![ello](../assets/example.svg)

[ello](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-first", LinkFirst("ello"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn self_reference_unlinked() {
    let src = r#"---
eip: 1234
---

EIP-1234

EIP-1234
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-first", LinkFirst("EIP-(1234)"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
