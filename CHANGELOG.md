# Changelog

## 0.9.0 - 2024-10-23

_Breaks compatibility with `--config`, the `Reporter` API, the `Lint` API, and the JavaScript API._

### Added

 - New lint `markdown-headings-space` reporting improperly spaced headings.  Thank you to [@0xRampey]!
 - Relevant suggestions in `markdown-rel-links` when linking to the license file or other proposals. Thank you to [@aslikaya] and [@JEAlfonsoP]!
 - New list `markdown-no-backticks` reporting when an EIP-like string is wrapped in backticks (eg. \`EIP-1234\`.) Thank you to [@VictoriaAde]!
 - New crate `eipw-snippets` for holding application-level diagnostic information.


[@0xRampey]: https://github.com/0xRampey
[@aslikaya]: https://github.com/aslikaya
[@JEAlfonsoP]: https://github.com/JEAlfonsoP
[@VictoriaAde]: https://github.com/VictoriaAde

### Changed

 - Preamble parsing moved into its own crate.
 - `markdown-link-first` now allows self-references without a link. Thank you to [@aslikaya]!
 - [`annotate-snippets`] updated to 0.11.4 (breaking changes to at least `Lint` and `Reporter` traits, and JavaScript API.)
 - [`comrak`] updated to 0.29 (breaking changes to `Lint`.)
 - Various dependency updates.

[`annotate-snippets`]: https://crates.io/crates/annotate-snippets

### Fixed

 - `markdown-link-first` no longer triggers inside image alt text. Thank you to [@rutefig]!

[@rutefig]: https://github.com/rutefig

## 0.8.0 - 2023-10-23

_Breaks compatibility with `--config` and `default_lints`._

### Changed

- Add `prefix` and `suffix` options to `preamble::requires_status`.
- Add `prefix` and `suffix` options to `preamble::proposal_ref`.
- Add `prefix` and `suffix` options to `markdown::link_status`.

## 0.7.0 - 2023-10-23

_Breaks compatibility with `--config` and `default_lints`._

### Changed

- Add `prefix` (eg. `eip-`) and `suffix` (eg. `.md`) options to
  `markdown::proposal_ref` lint.

### Fixed

- Added `--locked` to GitHub Workflow so tests don't grab newer version of
  crates.
