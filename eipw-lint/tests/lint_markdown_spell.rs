/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::Spell;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn correctly_spelled() {
    let src = r#"---
header: value1
---

Here is some text.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-spell",
            Spell {
                personal_dictionary: "",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn incorrectly_spelled() {
    let src = r#"---
header: value1
---

Here is ssome text.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-spell",
            Spell {
                personal_dictionary: "",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-spell]: the word `ssome` is misspelled
  |
5 | Here is ssome text.
  |         ^^^^^ incorrectly spelled
  |
"#
    );
}

#[tokio::test]
async fn incorrectly_spelled_in_code() {
    let src = r#"---
header: value1
---

Here is `ssome` text.
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-spell",
            Spell {
                personal_dictionary: "",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn incorrectly_spelled_in_code_block() {
    let src = r#"---
header: value1
---

```
Here is ssome text.
```
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-spell",
            Spell {
                personal_dictionary: "",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn correctly_spelled_on_boundary() {
    let src = r#"---
header: value1
---

**refe**rence
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-spell",
            Spell {
                personal_dictionary: "",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn incorrectly_spelled_on_boundary() {
    let src = r#"---
header: value1
---

**hello**world
"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-spell",
            Spell {
                personal_dictionary: "",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[markdown-spell]: the word `helloworld` is misspelled
  |
4 | /
5 | | **hello**world
  | |______________^ somewhere here
  |
  = warning: could not find a line number for this message
"#
    );
}
