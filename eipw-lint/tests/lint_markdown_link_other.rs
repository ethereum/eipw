/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::LinkOther;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn link_destination_missing_eip() {
    let src = r#"---
header: value1
---
[EIP-2](../assets/foo.txt)
"#;

    let reports = Linter::<Text<String>>::default()
       .clear_lints()
       .deny("markdown-link-other", LinkOther(r"^(EIP|ERC)-(\d+)\s*\S*$".to_string()))
       .check_slice(None, src)
       .run()
       .await
       .unwrap()
       .into_inner();
    assert_eq!(
        reports,
        r#"error[markdown-link-other]: link text does not match link destination
  |
4 | [EIP-2](../assets/foo.txt)
  |
  = info: link destinstion must match text EIP
"#
    );
}

#[tokio::test]
async fn link_eip_number_differs_from_text() {
    let src = r#"---
header: value1
---
[EIP-1](../assets/eip-2/foo.txt)
"#;

    let reports = Linter::<Text<String>>::default()
       .clear_lints()
       .deny("markdown-link-other", LinkOther(r"^(EIP|ERC)-(\d+)\s*\S*$".to_string()))
       .check_slice(None, src)
       .run()
       .await
       .unwrap()
       .into_inner();
    assert_eq!(reports, 
    r#"error[markdown-link-other]: link text does not match link destination
  |
4 | [EIP-1](../assets/eip-2/foo.txt)
  |
  = info: link destinstion must match text EIP
"#
    );
}

#[tokio::test]
async fn should_be_ignored() {
    let src = r#"---
header: value1
---
[EIP-2](../assets/eip-2/foo.txt)
"#;

    let reports = Linter::<Text<String>>::default()
       .clear_lints()
       .deny("markdown-link-other", LinkOther(r"^(EIP|ERC)-(\d+)\s*\S*$".to_string()))
       .check_slice(None, src)
       .run()
       .await
       .unwrap()
       .into_inner();
    assert_eq!(reports, "");
}