/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::{lints::markdown::HeadingFirst, reporters::Text, Linter};

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
        .deny("markdown-headings-only", HeadingFirst {})
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports.trim(),
        "error[markdown-headings-only]: Nothing is permitted between the preamble and the first heading"
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

``` python
adj_factor = max(1 - ((timestamp - parent.timestamp) // 10), -99)
child_diff = int(max(parent.difficulty + (parent.difficulty // BLOCK_DIFF_FACTOR) * adj_factor, min(parent.difficulty, MIN_DIFF)))
...
```

If `block.number >= BYZANTIUM_FORK_BLKNUM`, we change the first line to the following:

``` python
adj_factor = max((2 if len(parent.uncles) else 1) - ((timestamp - parent.timestamp) // 9), -99)
```
### Rationale

This new formula ensures that the difficulty adjustment algorithm targets a constant average rate of blocks produced including uncles, and so ensures a highly predictable issuance rate that cannot be manipulated upward by manipulating the uncle rate. A formula that accounts for the exact number of included uncles:
``` python
adj_factor = max(1 + len(parent.uncles) - ((timestamp - parent.timestamp) // 9), -99)
```
can be fairly easily seen to be (to within a tolerance of ~3/4194304) mathematically equivalent to assuming that a block with `k` uncles is equivalent to a sequence of `k+1` blocks that all appear with the exact same timestamp, and this is likely the simplest possible way to accomplish the desired effect. But since the exact formula depends on the full block and not just the header, we are instead using an approximate formula that accomplishes almost the same effect but has the benefit that it depends only on the block header (as you can check the uncle hash against the blank hash).

Changing the denominator from 10 to 9 ensures that the block time remains roughly the same (in fact, it should decrease by ~3% given the current uncle rate of 7%).

### References

1. EIP 100 issue and discussion: https://github.com/ethereum/EIPs/issues/100
2. https://bitslog.wordpress.com/2016/04/28/uncle-mining-an-ethereum-consensus-protocol-flaw/"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-headings-only", HeadingFirst {})
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
