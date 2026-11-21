use eipw_lint::lints::markdown::regex::{Mode, Regex};
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

fn lint() -> Regex<&'static str> {
    Regex {
        message: "prose must not contain consecutive spaces",
        mode: Mode::Excludes,
        pattern: r" {2,}",
    }
}

#[tokio::test]
async fn reports_consecutive_spaces_in_prose() {
    let src = "---\nheader: value\n---\n\nThis has  two spaces.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-re-consecutive-spaces", lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("error[markdown-re-consecutive-spaces]"));
    assert!(reports.contains("prose must not contain consecutive spaces"));
}

#[tokio::test]
async fn allows_markdown_hard_break_spaces() {
    let src = "---\nheader: value\n---\n\nFirst line.  \nSecond line.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-re-consecutive-spaces", lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn ignores_consecutive_spaces_in_code() {
    let src = "---\nheader: value\n---\n\n`two  spaces`\n\n```text\ntwo  spaces\n```\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-re-consecutive-spaces", lint())
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
