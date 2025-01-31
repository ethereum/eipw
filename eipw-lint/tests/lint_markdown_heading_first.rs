/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::{lints::markdown::HeadingFirst, reporters::Text, Linter};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn invalid_eip() {
    let src = r#"---
eip: 1234
---

This is some text that appears before the first heading. Authors sometimes try
to write an introduction or preface to their proposal here. We don't want to allow
this.

## Abstract

After the "Abstract" heading is the first place we want to allow text."#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-heading-first", HeadingFirst {})
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-heading-first]: Nothing is permitted between the preamble and the first heading
  |
5 | This is some text that appears before the first heading. Authors sometimes try
  |
"#
    );
}

#[tokio::test]
async fn valid_eip() {
    let src = r#"---
eip: 100
title: Change difficulty adjustment to target mean block time including uncles
author: Vitalik Buterin (@vbuterin)
type: Standards Track
category: Core
status: Final
created: 2016-04-28
---

### Specification

Currently, the formula to compute the difficulty of a block includes the following logic:
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-heading-first", HeadingFirst {})
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
