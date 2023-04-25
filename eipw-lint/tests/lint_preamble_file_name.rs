/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::preamble::FileName;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn unicode() {
    let src = r#"---
a1: Bánana
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-file-name",
            FileName {
                name: "a1",
                prefix: "hi-",
                suffix: ".txt",
            },
        )
        .check_slice(Some("foo.txt"), src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-file-name]: file name must reflect the preamble header `a1`
 --> foo.txt:2:4
  |
2 | a1: Bánana
  |    ^^^^^^^ this value
  |
  = help: this file's name should be `hi-Bánana.txt`
"#
    );
}

#[tokio::test]
async fn mismatch() {
    let src = r#"---
a1: value
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-file-name",
            FileName {
                name: "a1",
                prefix: "hi-",
                suffix: ".txt",
            },
        )
        .check_slice(Some("foo.txt"), src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        reports,
        r#"error[preamble-file-name]: file name must reflect the preamble header `a1`
 --> foo.txt:2:4
  |
2 | a1: value
  |    ^^^^^^ this value
  |
  = help: this file's name should be `hi-value.txt`
"#
    );
}

#[tokio::test]
async fn matching() {
    let src = r#"---
a1: value
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-file-name",
            FileName {
                name: "a1",
                prefix: "hi-",
                suffix: ".txt",
            },
        )
        .check_slice(Some("hi-value.txt"), src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn missing() {
    let src = r#"---
a1: value
---
hello world"#;

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-file-name",
            FileName {
                name: "a1",
                prefix: "hi-",
                suffix: ".txt",
            },
        )
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
