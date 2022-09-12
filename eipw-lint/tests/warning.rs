/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::Trim;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn warning() {
    let src = r#"---
header:value0
header1:value0
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .warn("preamble-trim", Trim)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"warning[preamble-trim]: preamble header values must begin with a space
  |
2 | header:value0
  |        - space required here
  |
3 | header1:value0
  |         - space required here
  |
"#,
    );
}
