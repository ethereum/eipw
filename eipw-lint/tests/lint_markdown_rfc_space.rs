/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::config::DefaultOptions;
use eipw_lint::reporters::{Count, Null};
use eipw_lint::Linter;

async fn errors_for(reference: &str) -> usize {
    let mut options = DefaultOptions::<&'static str>::default();
    options.modifiers.clear();
    options
        .lints
        .retain(|slug, _| slug == "markdown-re-rfc-space");

    let source = format!("---\neip: 1\n---\n{reference}\n");
    let reports = Linter::with_options(Count::new(Null), options)
        .check_slice(None, &source)
        .run()
        .await
        .unwrap();

    reports.counts().error
}

#[tokio::test]
async fn rejects_malformed_rfc_references() {
    for reference in ["RFC-1234", "RFC1234", "RFC  1234", "RFC\t1234", "RFC\n1234"] {
        assert_eq!(
            errors_for(reference).await,
            1,
            "expected an error for {reference:?}"
        );
    }
}

#[tokio::test]
async fn allows_exactly_one_space() {
    for reference in ["RFC 1234", "rfc 8174", "[RFC 2119](https://example.com/)"] {
        assert_eq!(
            errors_for(reference).await,
            0,
            "expected no error for {reference:?}"
        );
    }
}

#[tokio::test]
async fn ignores_inline_code() {
    assert_eq!(errors_for("`RFC1234`").await, 0);
}
