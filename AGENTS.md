# AGENTS.md

eipw is an Ethereum Improvement Proposal (EIP) linter (tagline: "one more than eipv"). Rust cargo workspace (MSRV in CI), MPL-2.0. Upstream Repo: https://github.com/ethereum/eipw

## Tools

This project uses [Serena](https://github.com/oraios/serena). If it's installed, activate it. If not, important project context is available in `.serena/memories/core.md`.

## Layout
- `src/` — root package `eipw`: clap CLI (`src/main.rs`), exit codes via `sysexits` (DataErr if any error reported, Config for bad config).
- `eipw-lint/` — the lint engine (main crate). Core in `src/lints.rs`, config in `src/config.rs`, lints in `src/lints/{preamble,markdown}/*.rs`, registry in `src/lints/known_lints.rs`, reporters in `src/reporters.rs`, fetch in `src/fetch.rs`.
- `eipw-lint/dictionaries` is a git submodule. The build breaks without `git submodule update --init`.
- `eipw-preamble/` — parses the leading `---` preamble block (`Preamble::parse`/`split`, `Field` API).
- `eipw-snippets/` — diagnostic types (`Message`/`Snippet`/`Annotation`/`Level`) re-exporting `annotate_snippets`; JSON-serializable. Dual-licensed MIT/Apache. All lint output flows through it.
- `eipw-lint-js/` — wasm-bindgen npm package exposing `lint()` + `format()` (Node `Fetch` via `node:fs/promises`). `pkg/` is **uncommitted** wasm-pack output — regenerate, never hand-edit.
- `docs/` — hand-maintained GitHub Pages site: one dir per lint slug (`index.html` + shared `main.css`). `.github/workflows/pages.yaml` uploads it verbatim — there is **no build step**; edit HTML directly.

## Commands
Prerequisite for any build/test: the spell-check dictionaries are a git submodule (`eipw-lint/dictionaries`, wooorm/dictionaries) that `spell.rs` `include_str!`s — the build fails without it:

    git submodule update --init --recursive

CI-parity set, from the workspace root (CI: check + tests on ubuntu & windows + fmt job):

    cargo check --all --all-features --locked
    cargo test --all --no-default-features --locked
    cargo test --all --all-features --locked
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- --deny warnings

Run the linter locally:

    cargo run -p eipw -- <files-or-dirs> [opts]

eipw-lint-js only (requires `cargo install wasm-pack --version <version> --locked`, version defined in CI):

    wasm-pack test --node --all --locked
    cd eipw-lint-js && wasm-pack build -t nodejs

## Definition of done

See `.serena/memories/task_completion.md`.

## Conventions

See `.serena/memories/conventions.md`.
