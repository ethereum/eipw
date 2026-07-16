/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::{ConfiguredUint, Uint, UintMessage};
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn valid() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
eip: 1234
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-eip", Uint("eip"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn invalid() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
eip: -1234
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-eip", Uint("eip"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-eip]: preamble header `eip` must be an unsigned integer
  |
6 | eip: -1234
  |     ^^^^^^ not a non-negative integer
  |
"#
    );
}

#[tokio::test]
async fn eip_to_be_assigned() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
eip: <to be assigned>
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-eip",
            ConfiguredUint {
                name: "eip",
                messages: Some(vec![UintMessage {
                    value: "<to be assigned>",
                    message: "preamble header `eip` is waiting for an assigned number",
                    label: Some("number has not been assigned yet"),
                }]),
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-eip]: preamble header `eip` is waiting for an assigned number
  |
6 | eip: <to be assigned>
  |     ^^^^^^^^^^^^^^^^^ number has not been assigned yet
  |
"#
    );
}

#[tokio::test]
async fn unicode() {
    let src = r#"---
header: value0
other-header: value
header: value1
foo: bar
eip: 1é234
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-eip", Uint("eip"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-eip]: preamble header `eip` must be an unsigned integer
  |
6 | eip: 1é234
  |     ^^^^^^ not a non-negative integer
  |
"#
    );
}
