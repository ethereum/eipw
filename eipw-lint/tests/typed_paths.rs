/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::fetch::Fetch;
use eipw_lint::lints::preamble::{FileName, RequiresStatus};
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

use pretty_assertions::assert_eq;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use typed_path::{Utf8TypedPath, Utf8TypedPathBuf};

const LAST_CALL: &str = "---\neip: 1000\nstatus: Last Call\nrequires: 20\n---\nhello world";
const DRAFT: &str = "---\neip: 20\nstatus: Draft\n---\nhello world";

// Paths are normalized so that lookups don't depend on how `..` gets joined.
struct Files(HashMap<Utf8TypedPathBuf, &'static str>);

impl Files {
    fn new<const N: usize>(files: [(Utf8TypedPath<'_>, &'static str); N]) -> Self {
        Self(
            files
                .into_iter()
                .map(|(path, content)| (path.normalize(), content))
                .collect(),
        )
    }
}

impl Fetch for Files {
    fn fetch(
        &self,
        path: Utf8TypedPathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        let result = match self.0.get(&path.normalize()) {
            Some(content) => Ok((*content).to_owned()),
            None => Err(std::io::ErrorKind::NotFound.into()),
        };

        Box::pin(async { result })
    }
}

async fn requires_status(files: Files, source: Utf8TypedPath<'_>) -> String {
    Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-requires-status",
            RequiresStatus {
                requires: "requires",
                status: "status",
                flow: vec![vec!["Draft"], vec!["Last Call"]],
            },
        )
        .set_fetch(files)
        .check_file(source)
        .run()
        .await
        .unwrap()
        .into_inner()
}

async fn file_name(source: Utf8TypedPath<'_>) -> String {
    Linter::<Text<String>>::default()
        .clear_lints()
        .deny(
            "preamble-file-name",
            FileName {
                name: "eip",
                format: "eip-{}",
            },
        )
        .set_fetch(Files::new([(source, LAST_CALL)]))
        .check_file(source)
        .run()
        .await
        .unwrap()
        .into_inner()
}

#[tokio::test]
async fn windows_sibling() {
    let source = Utf8TypedPath::windows(r"C:\eips\eip-1000.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::windows(r"C:\eips\eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> C:\eips\eip-1000.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn windows_relative_sibling() {
    let source = Utf8TypedPath::windows(r"eips\eip-1000.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::windows(r"eips\eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> eips\eip-1000.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn windows_index_sibling() {
    let source = Utf8TypedPath::windows(r"C:\eips\eip-1000\index.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::windows(r"C:\eips\eip-20\index.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> C:\eips\eip-1000\index.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn windows_relative_index_sibling() {
    let source = Utf8TypedPath::windows(r"eips\eip-1000\index.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::windows(r"eips\eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> eips\eip-1000\index.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn windows_forward_slashes() {
    let source = Utf8TypedPath::windows("eips/eip-1000.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::windows(r"eips\eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> eips/eip-1000.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn unix_index_sibling() {
    let source = Utf8TypedPath::unix("/eips/eip-1000/index.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::unix("/eips/eip-20/index.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> /eips/eip-1000/index.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn unix_sibling() {
    let source = Utf8TypedPath::unix("/eips/eip-1000.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::unix("/eips/eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> /eips/eip-1000.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn unix_backslash_in_directory() {
    let source = Utf8TypedPath::unix(r"foo\bar/eip-1000.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::unix(r"foo\bar/eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> foo\bar/eip-1000.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn unix_backslash_in_file_name() {
    let source = Utf8TypedPath::unix(r"./foo\bar.md");
    let files = Files::new([
        (source, LAST_CALL),
        (Utf8TypedPath::unix("./eip-20.md"), DRAFT),
    ]);

    assert_eq!(
        requires_status(files, source).await,
        r#"error[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`
 --> ./foo\bar.md:4:10
  |
4 | requires: 20
  |          ^^^ has a less advanced status
  |
  = help: valid `status` values for this proposal are: `Draft`
"#
    );
}

#[tokio::test]
async fn windows_file_name() {
    let source = Utf8TypedPath::windows(r"C:\eips\eip-1000.md");
    assert_eq!(file_name(source).await, "");
}

#[tokio::test]
async fn windows_index_file_name() {
    let source = Utf8TypedPath::windows(r"C:\eips\eip-1000\index.md");
    assert_eq!(file_name(source).await, "");
}

#[tokio::test]
async fn windows_index_file_name_mismatch() {
    let source = Utf8TypedPath::windows(r"C:\eips\eip-1001\index.md");
    assert_eq!(
        file_name(source).await,
        r#"error[preamble-file-name]: file name must reflect the preamble header `eip`
 --> C:\eips\eip-1001\index.md:2:5
  |
2 | eip: 1000
  |     ^^^^^ this value
  |
  = help: this file should be in a folder `eip-1000`
"#
    );
}

#[tokio::test]
async fn unix_backslash_file_name() {
    let source = Utf8TypedPath::unix(r"eips\eip-1000.md");
    assert_eq!(
        file_name(source).await,
        r#"error[preamble-file-name]: file name must reflect the preamble header `eip`
 --> eips\eip-1000.md:2:5
  |
2 | eip: 1000
  |     ^^^^^ this value
  |
  = help: this file's name should be `eip-1000.md`
"#
    );
}
