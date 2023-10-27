# Changelog

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
