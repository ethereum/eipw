/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::reporters::Text;
use eipw_lint::Linter;

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use tokio::fs;

#[tokio::test]
async fn eipv() -> std::io::Result<()> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("eipv");

    let mut entries = fs::read_dir(&root).await?;

    let mut checked = false;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        std::env::set_current_dir(entry.path())?;

        checked = true;

        let input_path = Path::new("input.md");
        let expected_path = Path::new("expected.txt");
        let valid_path = Path::new("valid.txt");

        let expected = match fs::read_to_string(&expected_path).await {
            Ok(s) if s.trim().is_empty() => panic!(
                "`{}` is empty, use `valid.txt` instead",
                expected_path.display()
            ),
            Ok(s) => s,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                if fs::metadata(valid_path).await?.is_file() {
                    String::new()
                } else {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        };

        println!("Testing {}...", entry.path().display());
        let reports = Linter::<Text<String>>::default()
            .check_file(&input_path)
            .run()
            .await
            .unwrap()
            .into_inner();

        assert_eq!(
            reports,
            expected,
            "\nIn eipv fixture `{}`, expected:\n{}\n\nInstead got:\n{}\n",
            entry.path().file_name().unwrap().to_string_lossy(),
            expected,
            reports,
        );
    }

    assert!(checked);

    Ok(())
}
