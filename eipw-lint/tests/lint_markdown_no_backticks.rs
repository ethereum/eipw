/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::NoBackticks;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn eip_in_backticks() {
    let src = r#"---
header: value1
---

hello

`EIP-1234`
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-no-backticks]: proposal references should not be in backticks
  |
7 | `EIP-1234`
  |
  = info: the pattern in question: `EIP-[0-9]+`
"#
    );
}

#[tokio::test]
async fn valid_code_in_backticks() {
    let src = r#"---
header: value1
---

hello

`ERC20` and `IERC7777`
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn multiple_eip_references() {
    let src = r#"---
header: value1
---

This document references `EIP-1234` and `EIP-5678` which should both be flagged.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("EIP-1234"));
    assert!(reports.contains("EIP-5678"));
}

#[tokio::test]
async fn eip_in_code_block() {
    let src = r#"---
header: value1
---

Here's some code:

```solidity
// This is fine because it's in a code block
function implementEIP1234() {
    // EIP-1234 implementation
}
```

But this `EIP-1234` should be flagged.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("EIP-1234"));
    assert_eq!(reports.matches("EIP-1234").count(), 1);  // Only one instance should be flagged
}

#[tokio::test]
async fn eip_in_mixed_context() {
    let src = r#"---
header: value1
---

| EIP | Description |
|-----|-------------|
| `EIP-1234` | Some description |

- Item 1: `EIP-5678`
- Item 2: Some `code` and `EIP-9012`

The function `doSomething()` implements `EIP-3456`.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("EIP-1234"));
    assert!(reports.contains("EIP-5678"));
    assert!(reports.contains("EIP-9012"));
    assert!(reports.contains("EIP-3456"));
}

#[tokio::test]
async fn mixed_code_and_eip_references() {
    let src = r#"---
header: value1
---

The function `implementERC20()` follows `EIP-20` standard.
The class `MyToken` implements `EIP-721` for NFTs.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should flag EIP references in backticks
    assert!(reports.contains("EIP-20"));
    assert!(reports.contains("EIP-721"));
    
    // The error message should mention backticks
    assert!(reports.contains("proposal references should not be in backticks"));
}

#[tokio::test]
async fn eip_in_image_alt_text() {
    let src = r#"---
header: value1
---

![This is `EIP-1234` in alt text](image.png)
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should still flag EIP references in backticks, even in alt text
    assert!(reports.contains("EIP-1234"));
}

#[tokio::test]
async fn self_reference_eip() {
    let src = r#"---
eip: 1234
title: Test EIP
---

This is `EIP-1234` which is the current EIP.
"#;

    let linter = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"));

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    // Should flag EIP references in backticks, even for self-references
    assert!(reports.contains("EIP-1234"));
}
