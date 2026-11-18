# EIPV Tests

The tests under `eipw-lint/tests/eipv` are exact input/output tests. For historical context, eipv was the EIP linter before eipw.

Each test lives in its own folder, for example `eipw-lint/tests/eipv/markdown-html-comments`, usually by lint. New tests should focus on a single lint at a time, though that is not a hard requirement.

The input (`input.md`) should exactly follow the EIP format, except for the specific lint(s) under test. Should a test require an additonal file, say to verify a hyperlink, it is included alongside `input.md` in the same directory.

The expected verbatim output from `eipw` (errors, warnings, etc.) goes in `output.txt`, and is generated manually by running `eipw` against `input.md`, verifying that it is correct.

Every newly added lint should have an eipv-style test, unless it is extremely impractical to do so.
