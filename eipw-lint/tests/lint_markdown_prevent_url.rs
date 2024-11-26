/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 use eipw_lint::lints::markdown::PreventUrlsNoBackticks;
 use eipw_lint::reporters::Text;
 use eipw_lint::Linter;
 
 #[tokio::test]
 async fn url_in_backticks() {
     let src = r#"---
 header: value1
 ---
 
 This is a test with a URL in backticks:
 
 `https://notallowed.com`
 "#;
 
     let linter = Linter::<Text<String>>::default()
         .clear_lints()
         .deny(
             "markdown-prevent-url",
             PreventUrlsNoBackticks {
                 allowed_domains: Vec::<&str>::new(),
             },
         );
 
     let reports = linter
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(
         reports,
         r#"error[markdown-prevent-url]: URLs are not allowed in backticks
   |
 8 | `https://notallowed.com`
   |
   = info: This URL must be hyperlinked or from an allowed domain: `https://notallowed.com`
 "#
     );
 }
 
 #[tokio::test]
 async fn valid_url_not_in_backticks() {
     let src = r#"---
 header: value1
 ---
 
 This is a valid URL in plain text:
 
 https://example.com
 "#;
 
     let reports = Linter::<Text<String>>::default()
         .clear_lints()
         .deny(
             "markdown-prevent-url",
             PreventUrlsNoBackticks {
                 allowed_domains: Vec::<&str>::new(),
             },
         )
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(reports, "");
 }
 