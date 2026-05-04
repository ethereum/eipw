/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::LinkTextFormat;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn valid_eip_link() {
    let src = r#"---
eip: 4444
---

[EIP-1234](https://eips.ethereum.org/EIPS/eip-1234)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-text-format", LinkTextFormat)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn invalid_format_wrong_prefix() {
    let src = r#"---
eip: 4444
---

[ERC-1234](https://eips.ethereum.org/EIPS/eip-1234)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-text-format", LinkTextFormat)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(
        reports.contains("link text for EIP references must be in the format `EIP-N` or `ERC-N`"),
        "Expected error not found in reports: {}",
        reports
    );
}

#[tokio::test]
async fn invalid_format_no_prefix() {
    let src = r#"---
eip: 4444
---

[1234](https://eips.ethereum.org/EIPS/eip-1234)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-text-format", LinkTextFormat)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(
        reports.contains("link text for EIP references must be in the format `EIP-N` or `ERC-N`"),
        "Expected error not found in reports: {}",
        reports
    );
}

#[tokio::test]
async fn ignores_non_eip_urls() {
    let src = r#"---
eip: 4444
---

[click here](https://example.com)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-text-format", LinkTextFormat)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn erc_link_valid() {
    let src = r#"---
eip: 4444
---

[ERC-20](https://ercs.ethereum.org/ERCS/erc-20)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-text-format", LinkTextFormat)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}