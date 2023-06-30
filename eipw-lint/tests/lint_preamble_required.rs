/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Required;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn one_missing() {
    let src = r#"---
a1: value
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-required", Required(vec!["a1", "b2"]))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        "error[preamble-required]: preamble is missing header(s): `b2`\n |\n |\n"
    );
}

#[tokio::test]
async fn two_missing() {
    let src = r#"---
a2: value
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-required", Required(vec!["a1", "b2"]))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-required]: preamble is missing header(s): `a1`, `b2`
 |
 |
"#
    );
}

#[tokio::test]
async fn none_missing() {
    let src = r#"---
b2: value
a1: value
header: value1
foo: bar
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-required", Required(vec!["a1", "b2"]))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
