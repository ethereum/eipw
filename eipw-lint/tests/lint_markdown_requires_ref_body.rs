/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::RequiresRefBody;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn missing_ref_single() {
    let src = r#"---
eip: 1
title: Test EIP
description: A test EIP
author: Test Author <test@example.com>
discussions-to: https://ethereum-magicians.org/t/test/1
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
requires: 4, 5, 6
---

## Abstract

Building on EIP-4 and EIP-6, we blah blah blah...
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-requires-ref-body",
            RequiresRefBody {
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
        r#"error[markdown-requires-ref-body]: proposals EIP-5 must be mentioned in the body
   |
11 | requires: 4, 5, 6
   |           ^^^^^^^ required here
   |
"#,
    );
}

#[tokio::test]
async fn all_refs_present() {
    let src = r#"---
eip: 1
title: Test EIP
description: A test EIP
author: Test Author <test@example.com>
discussions-to: https://ethereum-magicians.org/t/test/1
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
requires: 4, 5, 6
---

## Abstract

Building on EIP-4, EIP-5, and EIP-6, we blah blah blah...
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-requires-ref-body",
            RequiresRefBody {
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
async fn multiple_missing_refs() {
    let src = r#"---
eip: 1
title: Test EIP
description: A test EIP
author: Test Author <test@example.com>
discussions-to: https://ethereum-magicians.org/t/test/1
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
requires: 4, 5, 6, 7
---

## Abstract

Building on EIP-4, we blah blah blah...
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-requires-ref-body",
            RequiresRefBody {
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
        r#"error[markdown-requires-ref-body]: proposals EIP-5, EIP-6, EIP-7 must be mentioned in the body
   |
11 | requires: 4, 5, 6, 7
   |           ^^^^^^^^^^ required here
   |
"#,
    );
}

#[tokio::test]
async fn no_requires_field() {
    let src = r#"---
eip: 1
title: Test EIP
description: A test EIP
author: Test Author <test@example.com>
discussions-to: https://ethereum-magicians.org/t/test/1
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
---

## Abstract

Building on EIP-4, we blah blah blah...
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-requires-ref-body",
            RequiresRefBody {
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
async fn erc_format_recognized() {
    let src = r#"---
eip: 1
title: Test EIP
description: A test EIP
author: Test Author <test@example.com>
discussions-to: https://ethereum-magicians.org/t/test/1
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
requires: 20, 721
---

## Abstract

Building on ERC-20 and ERC-721, we blah blah blah...
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-requires-ref-body",
            RequiresRefBody {
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
