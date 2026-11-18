# Tech stack

- Rust (`workspace.package.rust-version`; CI pins with `dtolnay/rust-toolchain`). No rust-toolchain file.
- Workspace members: root package `eipw` (CLI binary) + `eipw-lint`, `eipw-lint-js`, `eipw-preamble`, `eipw-snippets`. Shared versions pinned in root `[workspace.dependencies]`.
- Markdown: comrak (default-features off; table/autolink/footnotes enabled).
- Diagnostics: `annotate_snippets` (re-exported by eipw-snippets); spelling via `zspell` (wooorm dictionaries submodule + `custom.dic`).
- CLI: clap derive + `sysexits`; config: TOML (`toml` crate) merged with defaults via `figment`.
- Errors: `snafu` derive; `thiserror` in the root crate; `educe` for selective Debug.
- WASM: wasm-bindgen + js-sys + serde-wasm-bindgen, built with wasm-pack (Node target). eipw-lint-js is published to npm as `eipw-lint-js`.
- Optional feature `schema-version`: schemars + olpc-cjson + sha3 + semver; canonical-JSON hash of the config schema as a semver build tag, exposed via `--schema-version`.
- Licensing: workspace MPL-2.0; eipw-snippets dual-licensed MIT/Apache (separate LICENSE files).
