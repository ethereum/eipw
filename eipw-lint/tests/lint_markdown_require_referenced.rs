/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::RequireReferenced;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn all_referenced() {
    let src = r#"---
requires: 44, 55
---

This proposal builds on EIP-44 and ERC-55.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
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
async fn one_missing() {
    let src = r#"---
requires: 4, 5, 6
---

Building on EIP-4 and EIP-6, we blah blah blah.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-req-ref-body]: proposals listed in preamble header `requires` must be referenced in the body
  |
2 | requires: 4, 5, 6
  |              ^ not referenced in body
  |
"#
    );
}

#[tokio::test]
async fn two_missing() {
    let src = r#"---
requires: 4, 5, 6
---

Building on EIP-5, we blah blah blah.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-req-ref-body]: proposals listed in preamble header `requires` must be referenced in the body
  |
2 | requires: 4, 5, 6
  |           ^     ^ not referenced in body
  |           |
  |           not referenced in body
  |
"#
    );
}

#[tokio::test]
async fn missing_field() {
    let src = r#"---
title: Some EIP
---

Building on EIP-4 and EIP-5.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
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
async fn ignores_code_block() {
    let src = r#"---
requires: 44
---

```solidity
// EIP-44 mention inside a code block does not count.
contract Foo {}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-req-ref-body]: proposals listed in preamble header `requires` must be referenced in the body
  |
2 | requires: 44
  |           ^^ not referenced in body
  |
"#
    );
}

#[tokio::test]
async fn ignores_inline_code() {
    let src = r#"---
requires: 44
---

A reference to `EIP-44` inside backticks does not count.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-req-ref-body]: proposals listed in preamble header `requires` must be referenced in the body
  |
2 | requires: 44
  |           ^^ not referenced in body
  |
"#
    );
}

#[tokio::test]
async fn erc_satisfies_eip_in_requires() {
    let src = r#"---
requires: 44
---

Builds on ERC-44.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-req-ref-body",
            RequireReferenced {
                requires: "requires",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
