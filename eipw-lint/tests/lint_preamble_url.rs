/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Url;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn unicode() {
    let src = r#"---
header: exámple.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-url", Url("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-url]: preamble header `header` is not a valid URL
  |
2 | header: exámple.com/foo?bar
  |        ^^^^^^^^^^^^^^^^^^^^ relative URL without a base
  |
"#,
    );
}

#[tokio::test]
async fn invalid() {
    let src = r#"---
header: example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-url", Url("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-url]: preamble header `header` is not a valid URL
  |
2 | header: example.com/foo?bar
  |        ^^^^^^^^^^^^^^^^^^^^ relative URL without a base
  |
"#,
    );
}

#[tokio::test]
async fn valid() {
    let src = r#"---
header: https://example.com/foo?bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-url", Url("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
