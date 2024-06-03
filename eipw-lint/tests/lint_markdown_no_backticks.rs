/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 use eipw_lint::lints::markdown::NoBackticks;
 use eipw_lint::reporters::Text;
 use eipw_lint::Linter;
 
 #[tokio::test]
 async fn eip_in_backticks() {
     let src = r#"---
 header: value1
 ---
 hello
 
 `EIP-1234`
 "#;
 
     let reports = Linter::<Text<String>>::default()
         .clear_lints()
         .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"))
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(
         reports,
         r#"error[markdown-no-backticks]: EIP references should not be in backticks
    |
  5 | `EIP-1234`
    |
    = info: the pattern in question: `EIP-[0-9]+`
  "#
     );
 }
 
 #[tokio::test]
 async fn valid_code_in_backticks() {
     let src = r#"---
 header: value1
 ---
 hello
 
 `ERC20` and `IERC7777`
 "#;
 
     let reports = Linter::<Text<String>>::default()
         .clear_lints()
         .deny("markdown-no-backticks", NoBackticks(r"EIP-[0-9]+"))
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(reports, "");
 }
 