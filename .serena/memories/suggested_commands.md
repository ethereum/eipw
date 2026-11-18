# Suggested commands

Prerequisite for any local build/test: the spell-check dictionaries submodule must be checked out:
    git submodule update --init --recursive

CI-parity commands (from workspace root):
    cargo check --all --all-features --locked
    cargo test --all --no-default-features --locked
    cargo test --all --all-features --locked
    cargo fmt --all -- --check
    cargo clippy --all-features -- --deny warnings

Run the linter locally:
    cargo run -p eipw -- <files-or-dirs> [opts]
    # opts: --format json | -D/-W/-A <lint-slug> | -c config.toml | --defaults | --list-lints

JS package (only when touching eipw-lint-js):
    cargo install wasm-pack --version <version> --locked
    wasm-pack test --node --all --locked
    cd eipw-lint-js && wasm-pack build -t nodejs
