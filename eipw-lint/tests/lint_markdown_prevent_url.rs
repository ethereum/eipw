/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

 use crate::lints::PreventUrlsNoBackticks;
 use crate::reporters::Text; // Assuming you have a similar reporter in your crate
 use crate::Linter;
 
 #[tokio::test]
 async fn url_with_backticks() {
     let src = r#"---
 header: value1
 ---
 
 Check this URL: `http://example.com/page?query=foo`
 "#;
 
     let linter = Linter::<Text<String>>::default()
         .clear_lints()
         .deny("markdown-prevent-url", PreventUrlsNoBackticks(r"https://example\.com"));
 
     let reports = linter
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(
         reports,
         r#"error[markdown-prevent-url]: URL containing backticks or not from allowed domain found
   |
 4 | Check this URL: `http://example.com/page?query=foo`
   |
   = info: avoid using backticks in URLs: `http://example.com/page?query=foo`
 "#
     );
 }
 
 #[tokio::test]
 async fn valid_url_no_backticks() {
     let src = r#"---
 header: value1
 ---
 
 Check this URL: http://example.com/page?query=foo
 "#;
 
     let reports = Linter::<Text<String>>::default()
         .clear_lints()
         .deny("markdown-prevent-url", PreventUrlsNoBackticks(r"https://example\.com"))
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(reports, "");
 }
 
 #[tokio::test]
 async fn url_with_allowed_domain_no_backticks() {
     let src = r#"---
 header: value1
 ---
 
 Check this URL: http://example.com/page?query=foo
 "#;
 
     let linter = Linter::<Text<String>>::default()
         .clear_lints()
         .deny("markdown-prevent-url", PreventUrlsNoBackticks(r"example\.com"));
 
     let reports = linter
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(reports, "");
 }
 
 #[tokio::test]
 async fn url_with_disallowed_domain() {
     let src = r#"---
 header: value1
 ---
 
 Check this URL: http://notallowed.com/page?query=foo
 "#;
 
     let linter = Linter::<Text<String>>::default()
         .clear_lints()
         .deny("markdown-prevent-url", PreventUrlsNoBackticks(r"allowed\.com"));
 
     let reports = linter
         .check_slice(None, src)
         .run()
         .await
         .unwrap()
         .into_inner();
 
     assert_eq!(
         reports,
         r#"error[markdown-prevent-url]: URL containing backticks or not from allowed domain found
   |
 4 | Check this URL: http://notallowed.com/page?query=foo
   |
   = info: avoid using backticks in URLs: `http://notallowed.com/page?query=foo`
 "#
     );
 }
 