/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::SectionText;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

fn copyright_lint() -> SectionText<&'static str> {
    SectionText {
        section: "Copyright",
        level: 2,
        exactly: "Copyright and related rights waived via [CC0](../LICENSE.md).",
    }
}

#[tokio::test]
async fn valid_at_end() {
    let src = r#"---
header: value1
---

## Abstract
This is the abstract.

## Copyright
Copyright and related rights waived via [CC0](../LICENSE.md).
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn invalid_with_bold_heading() {
    let src = r#"---
header: value1
---

## Cop**y**right
Copyright and related rights waived via [CC0](../LICENSE.md).
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-copyright]: the `Copyright` section must be the last content in the file
  |
5 | ## Cop**y**right
  | ^^^^^^^^^^^^^^^^ nothing may follow this section
  |
  = help: end the file with `## Copyright` followed immediately by `Copyright and related rights waived via [CC0](../LICENSE.md).`, with no other content after it
"#
    );
}

#[tokio::test]
async fn allows_trailing_blank_lines() {
    let src = r#"---
header: value1
---

## Abstract
This is the abstract.

## Copyright
Copyright and related rights waived via [CC0](../LICENSE.md).


"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn invalid_extra_text_after_waiver() {
    let src = r#"---
header: value1
---

## Abstract
This is the abstract.

## Copyright
Copyright and related rights waived via [CC0](../LICENSE.md).

Extra text.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-copyright]: the `Copyright` section must be the last content in the file
  |
8 | ## Copyright
  | ^^^^^^^^^^^^ nothing may follow this section
  |
  = help: end the file with `## Copyright` followed immediately by `Copyright and related rights waived via [CC0](../LICENSE.md).`, with no other content after it
"#
    );
}

#[tokio::test]
async fn invalid_section_after_waiver() {
    let src = r#"---
header: value1
---

## Abstract
This is the abstract.

## Copyright
Copyright and related rights waived via [CC0](../LICENSE.md).

## Appendix
Extra text.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-copyright]: the `Copyright` section must be the last content in the file
  |
8 | ## Copyright
  | ^^^^^^^^^^^^ nothing may follow this section
  |
  = help: end the file with `## Copyright` followed immediately by `Copyright and related rights waived via [CC0](../LICENSE.md).`, with no other content after it
"#
    );
}

#[tokio::test]
async fn invalid_wrong_waiver_text() {
    let src = r#"---
header: value1
---

## Abstract
This is the abstract.

## Copyright
All rights reserved.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-copyright]: the `Copyright` section must be the last content in the file
  |
8 | ## Copyright
  | ^^^^^^^^^^^^ nothing may follow this section
  |
  = help: end the file with `## Copyright` followed immediately by `Copyright and related rights waived via [CC0](../LICENSE.md).`, with no other content after it
"#
    );
}

#[tokio::test]
async fn missing_copyright_section_is_ignored() {
    let src = r#"---
header: value1
---

## Abstract
This is the abstract.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-copyright", copyright_lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
