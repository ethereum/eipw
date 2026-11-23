/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::TitleCase;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

async fn lint(src: &str, allowed: Vec<&'static str>) -> String {
    Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-title-case",
            TitleCase {
                name: "title",
                small_words: vec!["a", "and", "for", "in", "of", "the", "to"],
                allowed,
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner()
}

#[tokio::test]
async fn valid() {
    let src = r#"---
title: Proof of Stake for the EVM
---
hello world"#;

    assert_eq!(lint(src, vec![]).await, "");
}

#[tokio::test]
async fn missing() {
    let src = r#"---
header: lowercase words
---
hello world"#;

    assert_eq!(lint(src, vec![]).await, "");
}

#[tokio::test]
async fn lowercase_words() {
    let src = r#"---
title: SSZ transaction signature scheme
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: SSZ transaction signature scheme
  |            ^^^^^^^^^^^ ^^^^^^^^^ ^^^^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn small_word_first() {
    let src = r#"---
title: the Merge of the Chains
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: the Merge of the Chains
  |        ^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn small_word_last() {
    let src = r#"---
title: Accounts to Opt in
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: Accounts to Opt in
  |                        ^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn hyphenated() {
    let src = r#"---
title: Fork-choice Enforced storage-clear Refund
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: Fork-choice Enforced storage-clear Refund
  |                             ^^^^^^^^^^^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn identifiers() {
    for title in [
        "Add wallet_signIntendedValidatorData Method",
        "Account Code Restricted ecRecover",
        "eth/70 - Partial Block Receipt Lists",
        "snap/vhash and secp256k1 for zk-SNARK Proofs",
    ] {
        let src = format!("---\ntitle: {title}\n---\nhello world");
        assert_eq!(lint(&src, vec![]).await, "", "title: {title}");
    }
}

#[tokio::test]
async fn code_span() {
    let src = r#"---
title: Reserve `extension (0xae)` and `eth1data` fields
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: Reserve `extension (0xae)` and `eth1data` fields
  |                                                  ^^^^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn allowed() {
    let src = r#"---
title: etha - Sharded blocks Subprotocol
---
hello world"#;

    assert_eq!(
        lint(src, vec!["etha"]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: etha - Sharded blocks Subprotocol
  |                       ^^^^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn punctuation() {
    let src = r#"---
title: "Simple token, Simplified (and extended)"
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: "Simple token, Simplified (and extended)"
  |                ^^^^^                  ^^^^^^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}

#[tokio::test]
async fn unicode() {
    let src = r#"---
title: Hegotá überschrift für Élan
---
hello world"#;

    assert_eq!(
        lint(src, vec![]).await,
        r#"error[preamble-title-case]: preamble header `title` should be in title case
  |
2 | title: Hegotá überschrift für Élan
  |               ^^^^^^^^^^^ ^^^
  |
  = help: capitalize every word, except these when not first or last: a, and, for, in, of, the, to
"#,
    );
}
