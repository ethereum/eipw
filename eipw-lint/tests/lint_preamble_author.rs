/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Author;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn one_invalid() {
    let src = r#"---
header: Foo (
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-author", Author("header"))
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-author]: authors in the preamble must match the expected format
  |
2 | header: Foo (
  |        ^^^^^^ unrecognized author
  |
  = help: Try `Random J. User (@username)` for an author with a GitHub username.
  = help: Try `Random J. User <test@example.com>` for an author with an email.
  = help: Try `Random J. User` for an author without contact information.
error[preamble-author]: preamble header `header` must contain at least one GitHub username
  |
2 | header: Foo (
  |
"#,
    );
}

#[tokio::test]
async fn invalid_last() {
    let src = r#"---
header: User (@user), Foo (
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-author", Author("header"))
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-author]: authors in the preamble must match the expected format
  |
2 | header: User (@user), Foo (
  |                      ^^^^^^ unrecognized author
  |
  = help: Try `Random J. User (@username)` for an author with a GitHub username.
  = help: Try `Random J. User <test@example.com>` for an author with an email.
  = help: Try `Random J. User` for an author without contact information.
"#,
    );
}

#[tokio::test]
async fn invalid_first() {
    let src = r#"---
header: Foo (, User (@user)
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-author", Author("header"))
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-author]: authors in the preamble must match the expected format
  |
2 | header: Foo (, User (@user)
  |        ^^^^^^ unrecognized author
  |
  = help: Try `Random J. User (@username)` for an author with a GitHub username.
  = help: Try `Random J. User <test@example.com>` for an author with an email.
  = help: Try `Random J. User` for an author without contact information.
"#,
    );
}

#[tokio::test]
async fn only_email() {
    let src = r#"---
header: Foo <test@example.com>
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-author", Author("header"))
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-author]: preamble header `header` must contain at least one GitHub username
  |
2 | header: Foo <test@example.com>
  |
"#,
    );
}

#[tokio::test]
async fn valid() {
    let src = r#"---
header: Foo <test@example.com>, Bar (@bar), Random J. User
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("preamble-author", Author("header"))
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
