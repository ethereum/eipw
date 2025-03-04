/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::lints::markdown::{regex::Mode, Regex};
use eipw_lint::reporters::Text;
use eipw_lint::Linter;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn regex_smart_quotes() {
    // Using Unicode escape sequences for smart quotes
    let src = "---
header: value1
---

This document uses \u{201C}smart quotes\u{201D} which should be flagged.
";

    let linter = Linter::<Text<String>>::default().clear_lints().deny(
        "markdown-re-smart-quotes",
        Regex {
            mode: Mode::Excludes,
            pattern: r"[\u{201C}\u{201D}]|[\u{2018}\u{2019}]",
            message: "smart quotes are not allowed (use straight quotes instead)",
        },
    );

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("smart quotes are not allowed"));
}

#[tokio::test]
async fn regex_no_smart_quotes() {
    let src = "---
header: value1
---

This document uses \"straight quotes\" which are fine.
";

    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "markdown-re-smart-quotes",
            Regex {
                mode: Mode::Excludes,
                pattern: r"[\u{201C}\u{201D}]|[\u{2018}\u{2019}]",
                message: "smart quotes are not allowed (use straight quotes instead)",
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
async fn regex_smart_single_quotes() {
    // Using Unicode escape sequences for smart single quotes
    let src = "---
header: value1
---

This document uses \u{2018}smart single quotes\u{2019} which should also be flagged.
";

    let linter = Linter::<Text<String>>::default().clear_lints().deny(
        "markdown-re-smart-quotes",
        Regex {
            mode: Mode::Excludes,
            pattern: r"[\u{201C}\u{201D}]|[\u{2018}\u{2019}]",
            message: "smart quotes are not allowed (use straight quotes instead)",
        },
    );

    let reports = linter
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("smart quotes are not allowed"));
}
