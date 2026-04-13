/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![cfg(feature = "tokio")]
#![cfg(not(target_arch = "wasm32"))]

use eipw_lint::reporters::Text;
use eipw_lint::Linter;

use std::path::{Path, PathBuf};

use tokio::fs;

#[tokio::test]
async fn bug_12_requires_status_with_relative_path() {
    let temp = PathBuf::from(std::env::temp_dir()).join("eipw-bug-12-test");
    let _ = fs::remove_dir_all(&temp).await;
    fs::create_dir_all(&temp.join("EIPS")).await.unwrap();

    let eip_100 = temp.join("EIPS/eip-100.md");
    let eip_200 = temp.join("EIPS/eip-200.md");

    fs::write(
        &eip_100,
        r#"---
eip: 100
title: A sample proposal
description: This proposal is a sample
author: John Doe <john@example.com>
discussions-to: https://example.com/t/100
status: Draft
type: Standards Track
category: Core
created: 2020-01-01
requires: 200
---

## Abstract
This is the abstract.
"#,
    )
    .await
    .unwrap();

    fs::write(
        &eip_200,
        r#"---
eip: 200
title: Required proposal
description: A required proposal
author: Jane Doe <jane@example.com>
discussions-to: https://example.com/t/200
status: Draft
type: Standards Track
category: Core
created: 2020-01-01
---

## Abstract
Required.
"#,
    )
    .await
    .unwrap();

    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp).unwrap();

    let result = Linter::<Text<String>>::default()
        .allow("preamble-file-name")
        .check_file(Path::new("EIPS/eip-100.md"))
        .run()
        .await;

    std::env::set_current_dir(original_cwd).unwrap();
    let _ = fs::remove_dir_all(&temp).await;

    let reports = result.unwrap().into_inner();
    assert!(
        !reports.contains("ENOENT"),
        "should not fail with missing file: {}",
        reports
    );
    assert!(
        !reports.contains("preamble-requires-status"),
        "should not error on preamble-requires-status: {}",
        reports
    );
}
