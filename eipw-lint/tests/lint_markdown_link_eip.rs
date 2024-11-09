/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::LinkEip;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn eip_number_mismatch() {
    let src = r#"---
header: value1
---
[EIP-1](./eip-2.md)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1](./eip-2.md)
  |
  = help: use `[EIP-2](./eip-2.md)` instead
"#
    );
}

#[tokio::test]
async fn link_eip_has_no_section() {
    let src = r#"---
header: value1
---
[EIP-1: Foo](./eip-1.md)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1: Foo](./eip-1.md)
  |
  = help: use `[EIP-1](./eip-1.md)` instead
"#
    );
}

#[tokio::test]
async fn link_text_missing_eip() {
    let src = r#"---
header: value1
---
[Another Proposal](./eip-1.md)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#.+)?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [Another Proposal](./eip-1.md)
  |
  = help: use `[EIP-1](./eip-1.md)` instead
"#
    );
}

#[tokio::test]
async fn link_text_missing_section_description() {
    let src = r#"---
header: value1
---
[EIP-1](./eip-1.md#motivation)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1](./eip-1.md#motivation)
  |
  = help: use `[EIP-1: Motivation](./eip-1.md#motivation)` instead
"#
    );
}

#[tokio::test]
async fn link_text_with_bold() {
    let src = r#"---
header: value1
---
[EIP-1**EIP-1**](./eip-1.md)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1**EIP-1**](./eip-1.md)
  |
  = help: use `[EIP-1](./eip-1.md)` instead
"#
    );
}

#[tokio::test]
async fn link_text_extended_section_description_with_bold() {
    let src = r#"---
header: value1
---
[EIP-1: eip motivation**EIP-1: eip motivation**](./eip-1.md#eip-motivation)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1: eip motivation**EIP-1: eip motivation**](./eip-1.md#eip-motivation)
  |
  = help: use `[EIP-1: Eip motivation](./eip-1.md#eip-motivation)` instead
"#
    );
}

#[tokio::test]
async fn link_text_missing_extended_section_description_with_hyphen() {
    let src = r#"---
header: value1
---
[EIP-1](./eip-1.md#eip-motivation)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1](./eip-1.md#eip-motivation)
  |
  = help: use `[EIP-1: Eip motivation](./eip-1.md#eip-motivation)` instead
"#
    );
}

#[tokio::test]
async fn link_text_missing_extended_section_description_with_underscore() {
    let src = r#"---
header: value1
---
[EIP-1](./eip-1.md#eip_motivation)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-1](./eip-1.md#eip_motivation)
  |
  = help: use `[EIP-1: Eip motivation](./eip-1.md#eip_motivation)` instead
"#
    );
}

#[tokio::test]
async fn eip_number_mismatch_with_section() {
    let src = r#"---
header: value1
---
[EIP-2: Hello](./eip-1.md#abstract)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-2: Hello](./eip-1.md#abstract)
  |
  = help: use `[EIP-1: Abstract](./eip-1.md#abstract)` instead
"#
    );
}

#[tokio::test]
async fn eip_number_mismatch_extended_section_description_with_hyphen() {
    let src = r#"---
header: value1
---
[EIP-2: Hello](./eip-1.md#hello-abstract)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-2: Hello](./eip-1.md#hello-abstract)
  |
  = help: use `[EIP-1: Hello abstract](./eip-1.md#hello-abstract)` instead
"#
    );
}

#[tokio::test]
async fn eip_number_mismatch_extended_section_description_with_underscore() {
    let src = r#"---
header: value1
---
[EIP-2: Hello](./eip-1.md#hello_abstract)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [EIP-2: Hello](./eip-1.md#hello_abstract)
  |
  = help: use `[EIP-1: Hello abstract](./eip-1.md#hello_abstract)` instead
"#
    );
}

#[tokio::test]
async fn link_text_missing_eip_with_section() {
    let src = r#"---
header: value1
---
[Another Proposal](./eip-1.md#rationale)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-eip]: link text does not match link destination
  |
4 | [Another Proposal](./eip-1.md#rationale)
  |
  = help: use `[EIP-1: Rationale](./eip-1.md#rationale)` instead
"#
    );
}

#[tokio::test]
async fn should_be_ignored() {
    let src = r#"---
header: value1
---
[EIP-721's Motivation](./eip-721.md#motivation)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-link-eip",
            LinkEip(r"(eip-)([^.]*)\.md(#(.+))?$".to_string()),
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();
    assert_eq!(reports, "");
}
