use eipw_lint::lints::preamble::Spell;
use eipw_lint::reporters::Text;
use eipw_lint::Linter;

fn lint(personal_dictionary: &'static str) -> Spell<&'static str> {
    Spell {
        names: vec!["title", "description"],
        personal_dictionary,
    }
}

#[tokio::test]
async fn reports_misspelled_title() {
    let src = "---\ntitle: A ssimple proposal\ndescription: A clear description\n---\n\nBody.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-spell", lint(""))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("the word `ssimple` is misspelled"));
    assert!(reports.contains("incorrectly spelled"));
}

#[tokio::test]
async fn reports_misspelled_description() {
    let src = "---\ntitle: A simple proposal\ndescription: A cleer description\n---\n\nBody.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-spell", lint(""))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert!(reports.contains("the word `cleer` is misspelled"));
}

#[tokio::test]
async fn ignores_structured_fields_not_configured_for_spellcheck() {
    let src = "---\ntitle: A simple proposal\ndescription: A clear description\nauthor: Xqzz Person <xqzz@example.com>\ndiscussions-to: https://example.com/xqzz\n---\n\nBody.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-spell", lint(""))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}

#[tokio::test]
async fn honors_personal_dictionary() {
    let src = "---\ntitle: A Floop proposal\ndescription: A clear description\n---\n\nBody.\n";
    let reports = Linter::<Text<String>>::default()
        .clear_lints()
        .deny("preamble-spell", lint("Floop\n"))
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reports, "");
}
