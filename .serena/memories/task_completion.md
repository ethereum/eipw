# Task completion

A coding task is done when all of these pass from the workspace root:

    cargo fmt --all -- --check
    cargo clippy --all-features -- --deny warnings
    cargo test --all --no-default-features --locked
    cargo test --all --all-features --locked

(Parity extra: `cargo check --all --all-features --locked`.)

If `eipw-lint-js` changed, additionally run `wasm-pack test --node --all --locked` (wasm-pack required).

## New or Updated Lints

If a lint is added or modified, a coding task additionally requires that:
- The lint table in `README.md` is updated
- HTML documentation in `docs/` is added/updated
- An eipv-style test is added (see `mem:eipv_tests`)
