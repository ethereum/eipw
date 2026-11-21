/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![cfg(not(target_arch = "wasm32"))]

use eipw_lint::fetch::Fetch;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct TrackingFetch {
    fetched: Arc<Mutex<Vec<PathBuf>>>,
}

impl Fetch for TrackingFetch {
    fn fetch(&self, path: PathBuf) -> Pin<Box<dyn Future<Output = Result<String, io::Error>>>> {
        self.fetched.lock().unwrap().push(path.clone());
        let fut = async move { tokio::fs::read_to_string(&path).await };
        Box::pin(fut)
    }
}

#[tokio::test]
async fn fetch_proposals_uses_absolute_paths() -> io::Result<()> {
    let base = std::env::temp_dir().join(format!("eipw-test-{}", std::process::id()));
    let eips = base.join("eips");
    let other = base.join("other");

    tokio::fs::create_dir_all(&eips).await?;
    tokio::fs::create_dir_all(&other).await?;

    let input = r#"---
eip: 1234
title: A sample proposal
description: A sample description
author: John Doe (@johndoe)
discussions-to: https://ethereum-magicians.org/t/hello/1
status: Draft
type: Standards Track
category: Core
created: 2020-01-01
requires: 20
---

## Abstract
This is the abstract for the EIP.
"#;

    tokio::fs::write(eips.join("input.md"), input).await?;
    tokio::fs::write(eips.join("eip-20.md"), "---\nstatus: Draft\n---\n").await?;

    let original_cwd = std::env::current_dir()?;
    std::env::set_current_dir(&other)?;

    let tracking = TrackingFetch {
        fetched: Arc::new(Mutex::new(Vec::new())),
    };

    let result = Linter::<Text<String>>::default()
        .allow("preamble-file-name")
        .set_fetch(tracking.clone())
        .check_file(Path::new("../eips/input.md"))
        .run()
        .await;

    std::env::set_current_dir(original_cwd)?;

    // Clean up
    let _ = tokio::fs::remove_dir_all(&base).await;

    let _reports = result.unwrap().into_inner();
    let fetched = Arc::try_unwrap(tracking.fetched)
        .unwrap()
        .into_inner()
        .unwrap();

    // Verify that fetch_proposals requested absolute paths,
    // not relative paths dependent on the working directory.
    let proposal_path = fetched
        .iter()
        .find(|p| p.file_name().map(|f| f == "eip-20.md").unwrap_or(false))
        .expect("eip-20.md should have been fetched");

    assert!(
        proposal_path.is_absolute(),
        "fetch_proposals should use absolute paths, got: {}",
        proposal_path.display()
    );

    Ok(())
}
