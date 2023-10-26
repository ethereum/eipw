# Changelog

## 0.7.0 - 2023-10-23

_Breaks compatibility with `--config` and `default_lints`._

### Changed

- Add `prefix` (eg. `eip-`) and `suffix` (eg. `.md`) options to
  `markdown::proposal_ref` lint.

### Fixed

- Added `--locked` to GitHub Workflow so tests don't grab newer version of
  crates.
