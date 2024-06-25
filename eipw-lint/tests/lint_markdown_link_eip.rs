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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
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
  = info: link text should match `[EIP|ERC-2]`
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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
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
  = info: link text should match `[EIP|ERC-1]`
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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
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
  = info: link text should match `[EIP|ERC-1]`
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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
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
  = info: link text should match `[EIP|ERC-1<section-description>]`
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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
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
  = info: link text should match `[EIP|ERC-1<section-description>]`
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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
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
  = info: link text should match `[EIP|ERC-1<section-description>]`
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
       .deny("markdown-link-eip", LinkEip(r"eip-([^.]*)\.md(#.+)?$".to_string()))
       .check_slice(None, src)
       .run()
       .await
       .unwrap()
       .into_inner();
    assert_eq!(reports, "");
}