# Changelog

## 0.10.0-dev - [Unreleased]

_Breaks compatibility with `--config`._

### Added

 - New lint `markdown-heading-first`. Thank you to [@Abeeujah]! ([#117])
 - New lint `markdown-no-backticks`. Thank you to [@apeaircreative]! ([#121])
 - New lint `markdown-spell`. ([`1d595cc`])

### Changed

 - Various improvements for configuration error reporting. ([`9812014`],
   [`24e5142`])
 - Reduced CI noise. ([`ce98cd2`])
 - Report message improvements. ([`8fab180`])
 - Replace `prefix` / `suffix` options with templates. ([`d7dc36b`])
 - `--config` now extends the default configuration instead of replacing it. ([`b693091`])

[`24e5142`]: https://github.com/ethereum/eipw/commit/24e51422403126d3e78a3d9bb1df80c29cc4b085
[`9812014`]: https://github.com/ethereum/eipw/commit/9812014159fd609c6c3addb21b27f87257ff29c0
[`ce98cd2`]: https://github.com/ethereum/eipw/commit/ce98cd20a192199b4d5be5089af7207f61fb8495
[`8fab180`]: https://github.com/ethereum/eipw/commit/8fab1807926774579d7b40ca2d853ea5054f4985
[`d7dc36b`]: https://github.com/ethereum/eipw/commit/d7dc36b6d339f4416d11d40427fa855fd0ac0c0e
[#117]: https://github.com/ethereum/eipw/pull/117
[@Abeeujah]: https://github.com/Abeeujah
[#121]: https://github.com/ethereum/eipw/pull/121
[@apeaircreative]: https://github.com/apeaircreative
[`b693091`]: https://github.com/ethereum/eipw/commit/b6930911a0b91fd71fc16ca924c617f4bdec9b2d
[`1d595cc`]: https://github.com/ethereum/eipw/commit/1d595cc61ba4f92838096429211d1b475b546d37

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
