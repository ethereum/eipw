/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::NoEmphasisKeywords;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn detects_bold_keywords() {
    let src = r#"---
eip: 1
title: Test EIP
description: Test description
author: Test Author
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
---

## Abstract

This implementation **MUST** be followed.

## Specification

This feature **SHOULD** be implemented by all clients.

## Rationale

This is **REQUIRED** for compatibility.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-emphasis-keywords", NoEmphasisKeywords)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("uppercase keywords should not be formatted with bold emphasis"));
    assert!(reports.contains("uppercase keyword `MUST` found in bold text"));
    assert!(reports.contains("uppercase keyword `SHOULD` found in bold text"));
    assert!(reports.contains("uppercase keyword `REQUIRED` found in bold text"));
}

#[tokio::test]
async fn detects_italic_keywords() {
    let src = r#"---
eip: 1
title: Test EIP
description: Test description
author: Test Author
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
---

## Abstract

This implementation *SHALL* be followed.

## Specification

This feature *MAY* be implemented by clients.

## Rationale

This is *OPTIONAL* for some implementations.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-emphasis-keywords", NoEmphasisKeywords)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("uppercase keywords should not be formatted with italic emphasis"));
    assert!(reports.contains("uppercase keyword `SHALL` found in italic text"));
    assert!(reports.contains("uppercase keyword `MAY` found in italic text"));
    assert!(reports.contains("uppercase keyword `OPTIONAL` found in italic text"));
}

#[tokio::test]
async fn allows_plain_keywords() {
    let src = r#"---
eip: 1
title: Test EIP
description: Test description
author: Test Author
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
---

## Abstract

This implementation MUST be followed.

## Specification

This feature SHOULD be implemented by all clients.

## Rationale

This is REQUIRED for compatibility. It MAY also be OPTIONAL in some cases.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-emphasis-keywords", NoEmphasisKeywords)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn allows_emphasis_without_keywords() {
    let src = r#"---
eip: 1
title: Test EIP
description: Test description
author: Test Author
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
---

## Abstract

This is **important** and *emphasized* text.

## Specification

**Bold** and *italic* formatting is allowed for other words.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-emphasis-keywords", NoEmphasisKeywords)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn detects_any_uppercase_keywords() {
    let src = r#"---
eip: 1
title: Test EIP
description: Test description
author: Test Author
status: Draft
type: Standards Track
category: Core
created: 2023-01-01
---

## Abstract

This is **IMPORTANT** information.

## Specification

The **API** should be used carefully.

## Rationale

Use the *HTTP* protocol for this.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-emphasis-keywords", NoEmphasisKeywords)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("uppercase keywords should not be formatted with bold emphasis"));
    assert!(reports.contains("uppercase keyword `IMPORTANT` found in bold text"));
    assert!(reports.contains("uppercase keyword `API` found in bold text"));
    assert!(reports.contains("uppercase keywords should not be formatted with italic emphasis"));
    assert!(reports.contains("uppercase keyword `HTTP` found in italic text"));
}
