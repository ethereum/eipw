/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::HeadingsSpace;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn normal_headings() {
    let src = r#"---
header: value1
---

##Banana
####Mango
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-headings-space", HeadingsSpace {})
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-headings-space]: Space missing in header
  |
5 | ##Banana
  |  ^ space required here
  |
6 | ####Mango
  |    ^ space required here
  |
"#
    );
}

#[tokio::test]
async fn abnormal_heading() {
    let src = r#"---
header: value1
---

##B#an#ana
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-headings-space", HeadingsSpace {})
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-headings-space]: Space missing in header
  |
5 | ##B#an#ana
  |  ^ space required here
  |
"#
    );
}

#[tokio::test]
async fn not_headings() {
    let src_str = r#"---
header: value1
---

*Hello*#world
`#world`
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-headings-space", HeadingsSpace {})
        .check_slice(None, src_str)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
