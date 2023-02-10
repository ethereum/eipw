/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::List;
use eipw_lint::reporters::{AdditionalHelp, Text};
use eipw_lint::Linter;

#[tokio::test]
async fn two_errors() {
    let src = r#"---
header: , example.com/foo?bar,
---
hello world"#;

    let reporter = AdditionalHelp::new(Text::<String>::default(), |f: &str| {
        Ok(format!("here {}", f))
    });

    let reports = Linter::new(reporter)
        .clear_lints()
        .deny("preamble-list", List("header"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-list]: preamble header `header` cannot have empty items
  |
2 | header: , example.com/foo?bar,
  |        ^ this item is empty
  |
  = help: here preamble-list
error[preamble-list]: preamble header `header` cannot have empty items
  |
2 | header: , example.com/foo?bar,
  |                              ^ this item is empty
  |
"#,
    );
}
