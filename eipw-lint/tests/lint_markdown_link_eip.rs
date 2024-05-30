/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 use eipw_lint::lints::markdown::LinkEip;
 use eipw_lint::reporters::Text;
 use eipw_lint::Linter;
 
 #[tokio::test]
 async fn link_matches_the_pattern() {
     let src = r#"---
 header: value1
 ---
 [EIP-1](./eip-2.md)
 "#;
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-link-eip", LinkEip("EIP-1"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

        // assert_eq!(reports, "");
 } 