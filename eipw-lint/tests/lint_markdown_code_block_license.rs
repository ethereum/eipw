/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::CodeBlockLicense;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn wrong_license() {
    let src = r#"---
header: value1
---

```solidity
// SPDX-License-Identifier: MIT

interface FooToken {}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-solidity-license",
            CodeBlockLicense {
                language: "solidity",
                license: "CC0-1.0",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-solidity-license]: code block of type `solidity` must use license `CC0-1.0`, not `MIT`
  |
5 | / ```solidity
6 | | // SPDX-License-Identifier: MIT
7 | |
8 | | interface FooToken {}
9 | | ```
  | |___^
  |
"#
    );
}

#[tokio::test]
async fn correct_license() {
    let src = r#"---
header: value1
---

```solidity
// SPDX-License-Identifier: CC0-1.0

interface FooToken {}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-solidity-license",
            CodeBlockLicense {
                language: "solidity",
                license: "CC0-1.0",
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
async fn no_spdx_header() {
    let src = r#"---
header: value1
---

```solidity
interface FooToken {}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-solidity-license",
            CodeBlockLicense {
                language: "solidity",
                license: "CC0-1.0",
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
async fn different_language_is_ignored() {
    let src = r#"---
header: value1
---

```rust
// SPDX-License-Identifier: MIT
fn main() {}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-solidity-license",
            CodeBlockLicense {
                language: "solidity",
                license: "CC0-1.0",
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
async fn block_comment_spdx() {
    let src = r#"---
header: value1
---

```solidity
/* SPDX-License-Identifier: MIT */
interface FooToken {}
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-solidity-license",
            CodeBlockLicense {
                language: "solidity",
                license: "CC0-1.0",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("must use license `CC0-1.0`, not `MIT`"));
}

#[tokio::test]
async fn language_with_info_string_attrs() {
    // Some markdown renderers allow extra info on the fence, e.g. ``` solidity {.foo}
    let src = "---
header: value1
---

```solidity {.line-numbers}
// SPDX-License-Identifier: MIT
interface FooToken {}
```
";

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-solidity-license",
            CodeBlockLicense {
                language: "solidity",
                license: "CC0-1.0",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("must use license `CC0-1.0`, not `MIT`"));
}
