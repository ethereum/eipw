use eipw_lint::lints::markdown::SectionUnique;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

#[tokio::test]
async fn reports_duplicate_level_two_sections() {
    let src = "---\nheader: value\n---\n\n## Abstract\n\nOne.\n\n## Abstract\n\nTwo.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-duplicate-section", SectionUnique)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("duplicate section `Abstract`"));
    assert!(reports.contains("first occurrence"));
    assert!(reports.contains("duplicate occurrence"));
}

#[tokio::test]
async fn allows_distinct_sections() {
    let src = "---\nheader: value\n---\n\n## Abstract\n\nOne.\n\n## Specification\n\nTwo.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-duplicate-section", SectionUnique)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn ignores_repeated_nested_headings() {
    let src = "---\nheader: value\n---\n\n## Specification\n\n### Example\n\nOne.\n\n### Example\n\nTwo.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("markdown-no-duplicate-section", SectionUnique)
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
