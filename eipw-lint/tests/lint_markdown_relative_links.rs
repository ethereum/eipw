/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::RelativeLinks;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn inline_link_with_scheme() {
    let src = r#"---
header: value1
---

[hi](https://example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("markdown-rel", RelativeLinks)
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hi](https://example.com/)
  |
"#
    );
}

#[tokio::test]
async fn inline_link_protocol_relative() {
    let src = r#"---
header: value1
---

[hi](//example.com/)
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("markdown-rel", RelativeLinks)
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | [hi](//example.com/)
  |
"#
    );
}

#[tokio::test]
async fn inline_link_root_relative() {
    let src = r#"---
header: value1
---

Hello [hi](/foo)!
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("markdown-rel", RelativeLinks)
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | Hello [hi](/foo)!
  |
"#
    );
}

#[tokio::test]
async fn inline_link_relative() {
    let src = r#"---
header: value1
---

Hello [hi](./foo/bar)!
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("markdown-rel", RelativeLinks)
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn reference_link_with_scheme() {
    let src = r#"---
header: value1
---

Hello [hi][hello]!

[hello]: https://example.com
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("markdown-rel", RelativeLinks)
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-rel]: non-relative link or image
  |
5 | Hello [hi][hello]!
  |
"#
    );
}

#[tokio::test]
async fn reference_link_relative() {
    let src = r#"---
header: value1
---

Hello [hi][hello]!

[hello]: ./hello-world
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .add_lint("markdown-rel", RelativeLinks)
        .check(src)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
