# eipw-lint — lint engine

## Core types (`src/lints.rs`)
- `Lint` trait: `find_resources(&FetchContext)` (declare proposals to fetch; string sources may not fetch) + `lint(slug, &Context)`.
- `Context<'a,'b>`: `preamble()`, `body()` (comrak `AstNode`), `body_source()`, `origin()` / `origin_path()`, `annotation_level()`, `report(Message)`, `proposal(u32)` (parsed context of a referenced EIP), `ast_source(ast)` / `ast_lines(ast)` / `ast_snippet(ast, level, label)` for comrak node spans.
- `Linter<R: Reporter>` builder: `check_file` / `check_slice`, `warn` / `deny` / `allow` (panics on unknown slug) / `clear_lints`, `modify`, `set_fetch`, async `run()`. Panics if no lints or no sources. Lints run sorted by slug.
- `process()` in `lints.rs` splits preamble, parses body with comrak (table/autolink/footnotes enabled), and **shifts all comrak line positions by the preamble line count** — lint authors never deal with that offset.
- `tree.rs`: comrak traversal helpers (`TraverseExt`, `Visitor`, `Next`).

## Config (`src/config.rs`)
- `Options<M, L>`: `lints: HashMap<String, Override<L>>`, `modifiers: Vec<M>`, `fetch: Option<FetchOptions>` (default `proposal_format = "eip-{}"`). TOML/serde + figment; `DefaultOptions<S> = Options<DefaultModifier<S>, DefaultLint<S>>`; `Default` = all default lints enabled + Stagnant/Withdrawn status modifiers (downgrade to warning).
- `Override<T>`: enable/disable wrapper; TOML `enabled = false` to disable.
- `DefaultLint<S>` enum (`src/lints/known_lints.rs`): one variant per lint, serde `tag = "kind"`, `rename_all = "kebab-case"`; each variant wraps a struct from `lints/preamble/*` or `lints/markdown/*`. Keep `as_inner` / `map_to_str` / `From<DefaultLint<&str>> for DefaultLint<String>` arms in sync.
- Modifiers: `DefaultModifier` enum in `src/modifiers.rs`; `Modifier` mutates `LintSettings` (annotation level) per file.
- Reporters (`src/reporters.rs`): `Text`, `Json`, `Null`, `Count`, `AdditionalHelp` (appends "see https://ethereum.github.io/eipw/<slug>/" per lint id).
- Fetch (`src/fetch.rs` + `fetch/tokio.rs`): referenced EIPs resolved relative to the source file's dir (parent dir if source is `index.md`); tries `eip-<n>.md` then `eip-<n>/index.md`, panics if both exist.
- Paths are `typed_path::Utf8TypedPath` / `Utf8TypedPathBuf`, never `std::path`: WASM builds always get Unix semantics from `std::path`, so the caller states whether a path is Unix or Windows (`eipw-lint-js` asks Node.js for its separator; the CLI uses `Utf8NativePath`). String origins from `check_slice` follow the compilation target.

## Lint modules
- `src/lints/preamble/*.rs`: author, date, file_name, length, list, no_duplicates, one_of, order, proposal_ref, regex, require_referenced, required, required_if_eq, requires_status, trim, uint (+UintList), url.
- `src/lints/markdown/*.rs`: heading_first, headings_space, html_comments, json_schema (csl-json: `citation.json` + `csl-data.json`), link_first, link_status, no_backticks, proposal_ref, regex, relative_links, section_order, section_required, section_text, spell.
- `src/custom.dic`: extra spell words, concatenated with the submodule dictionary in `spell.rs` (loaded via `include_str!`, LRU-cached per personal dictionary).

## Report pattern (follow this in new lints)
`ctx.report(ctx.annotation_level().title(label).id(slug).snippet(Snippet::source(slice).line_start(line).origin_opt(ctx.origin()).annotation(ctx.annotation_level().span_utf8(slice, start, len).label(text))))`
Spans are offsets **within the snippet slice**, not the whole file. `span_utf8` (LevelExt in lints.rs) rounds the end to a char boundary.

## Tests
- Per-lint integration tests: `tests/lint_<name>.rs`, async `#[tokio::test]`, `pretty_assertions`, assert on rendered text output.
- Fixture harness `tests/eipv.rs` (guarded by feature `tokio`, not wasm): each dir under `tests/eipv/<case>/` needs `input.md` + either `expected.txt` (exact diagnostics; must be non-empty) or `valid.txt` (expects clean). Harness uses the default `Linter` and `allow("preamble-file-name")`. Corpus mirrors eipv's test suite; referenced EIP files (e.g. `eip-20.md`) live in the same dir.
- Feature flags: `schema-version` (schemars), `backtrace` (snafu), `tokio` (fetch impl).
