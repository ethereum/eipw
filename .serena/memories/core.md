# eipw — Ethereum Improvement Proposal (EIP) validator ("one more than eipv")

Rust cargo workspace; the root package `eipw` is also the CLI binary. License MPL-2.0. Repo: https://github.com/ethereum/eipw

## Crates
- `eipw` (root, `src/main.rs`) — clap CLI: `SOURCES...`, `--format text|json`, `--deny/-D`, `--warn/-W`, `--allow/-A`, `--config/-c` (TOML), `--defaults`, `--list-lints`, `--schema-version` (feature `schema-version`). Exit codes via `sysexits` (`DataErr` when any error reported, `Config` for bad config). Has `cfg(target_arch = "wasm32")` stubs.
- `eipw-lint` — the lint engine (the main crate). See `mem:eipw-lint/core` for architecture, lint layout, report pattern, and test conventions.
- `eipw-preamble` — parses the leading `---` preamble block: `Preamble::split`, `Preamble::parse`; `Field` API: `name()`, `value()`, `source()`, `line_start()`, `by_name()`.
- `eipw-snippets` — diagnostic types `Message`/`Snippet`/`Annotation`/`Level` (Error, Warning, Info, Note, Help), re-exports `annotate_snippets`. Everything a lint reports goes through this; JSON-serializable.
- `eipw-lint-js` — wasm-bindgen npm package exposing `lint()` + `format()`; Node `Fetch` via `node:fs/promises`. `pkg/` is committed wasm-pack output.

## Invariants
- Lint slugs are kebab-case, prefixed `preamble-` or `markdown-`. The canonical list = README table + `eipw-lint/src/config.rs::default_lints()`.
- Default lint/modifier configuration lives in `eipw-lint/src/config.rs` (`default_lints()`, `default_modifiers()`), not in the lint modules.
- `eipw-lint/dictionaries` is a **git submodule** (wooorm/dictionaries). The build breaks without `git submodule update --init`.
- `docs/` is a hand-maintained GitHub Pages site (one dir per lint slug with `index.html` + shared `main.css`); `.github/workflows/pages.yaml` uploads it verbatim — there is no build step.
- CI (`.github/workflows/test.yaml`): rust-toolchain via `dtolnay/rust-toolchain`, checkout with `submodules: true`.
- See `mem:tech_stack` about the toolchain/frameworks, `mem:conventions` for style, `mem:suggested_commands` for useful shell commands including CI parity, and `mem:task_completion` for the definition of done.

## Testing

- When working with eipv-style tests (under `eipw-lint/tests/eipv`), see `mem:eipv_tests`. Every new lint should have at least one eipv-style test.
