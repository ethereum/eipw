# Conventions

- Every Rust source file starts with the 5-line MPL-2.0 license header block.
- Errors: `snafu` derive enums, `#[non_exhaustive]`, `#[snafu(context(false))]` where context adds nothing; foreign errors wrapped via `Error::custom`.
- Lint structs are generic over their string type `S` (typically `String` in config, `&'static str` in defaults) with bounds like `S: Display + Debug + AsRef<str>`; keep `#[derive(Serialize, Deserialize)]` + `#[cfg_attr(feature = "schema-version", derive(schemars::JsonSchema))]` on all config types; serde uses `kebab-case` tags/renames and `Cow<'a, str>` for zero-copy.
- Adding a lint requires touching, in order: (1) new struct + `impl Lint` in `eipw-lint/src/lints/{preamble,markdown}/`, (2) `DefaultLint` variant in `lints/known_lints.rs` **plus all three match arms** (`as_inner`, `map_to_str`, `From<&str> for String`), (3) entry in `config.rs::default_lints()` with its kebab-case slug, (4) test in `eipw-lint/tests/lint_<name>.rs`, (5) row in README lint table, (6) `docs/<slug>/index.html` page.
- HTML documentation in `docs/<slug>/index.html` must include example output, and example output must be updated when lints change.
- Lints call `ctx.report(...)` with `.id(slug)`; the `AdditionalHelp` reporter derives the docs URL from the slug — never hardcode URLs in lints.
- Lint slugs: `preamble-*` / `markdown-*`, kebab-case.
- The comrak AST body is offset by the preamble length by the engine (`process()` in `lints.rs`); do not double-compensate in lints. `Context::line_index` / `span_utf8` exist for converting between lines/chars — use them, spans must land on char boundaries.
- Tests: async `#[tokio::test]`, `pretty_assertions::assert_eq`, fixture-based expected-output style (exact rendered diagnostics in `.txt`).
- `eipw-lint`'s Cargo `exclude` keeps `dictionaries` out of the published crate except license + the English `index.aff`/`index.dic`.
- Document only the current state of the code; never document what was there before.
- Comments must not describe what the code is doing; only why it is doing it (and even then, only rarely).
- Comments must be succinct, used sparingly, and written in full sentences.
- Commit messages should be succinct with **no** conventional commit prefix.
