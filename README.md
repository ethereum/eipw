eipw
====

The [EIP] validator that's one more than `eipv`.

[EIP]: https://eips.ethereum.org/

## Demo

### Example EIP

```markdown
---
eip: 2
description: A really short example of an EIP.
title: Sample of an EIP
author: Sam Wilson (@SamWilsn)
discussions-to: https://example.com/
status: Living
type: Meta
created: 2022-06-30
---

## Specification

Implementers of this EIP must...

## Abstract

This is an abstract!
```

### Output

```
error[markdown-order-section]: section `Specification` must come after `Motivation`
  --> /tmp/demo.md
   |
12 | ## Specification
   |
error[preamble-order]: preamble header `description` must come after `title`
 --> /tmp/demo.md
  |
3 | description: A really short example of an EIP.
  |
```

## Lints

| id                                  | Description                                                                                               |
|-------------------------------------|-----------------------------------------------------------------------------------------------------------|
| `markdown-order-section`            | Checks that there are no extra sections and that sections are in the correct order.                       |
| `preamble-author`                   | Checks that the author header is correctly formatted, and that there is at least one GitHub user listed.  |
| `preamble-date-created`             | Checks that the `created` header is a date.                                                               |
| `preamble-date-last-call-deadline`  | Checks that the `last-call-deadline` header is a date.                                                    |
| `preamble-discussions-to`           | Checks that the `discussions-to` header is a valid URL.                                                   |
| `preamble-eip`                      | Checks that the `eip` header is a non-negative integer.                                                   |
| `preamble-enum-category`            | Checks that the `category` header is a recognized value.                                                  |
| `preamble-enum-status`              | Checks that the `status` header is a recognized value.                                                    |
| `preamble-enum-type`                | Checks that the `type` header is a recognized value.                                                      |
| `preamble-len-description`          | Checks that the `description` header isn't too long.                                                      |
| `preamble-len-title`                | Checks that the `title` header isn't too long.                                                            |
| `preamble-list-author`              | Checks that the `author` header is a correctly formatted comma-separated list.                            |
| `preamble-list-requires`            | Checks that the `requires` header is a correctly formatted comma-separated list.                          |
| `preamble-no-dup`                   | Checks that there are no duplicate headers.                                                               |
| `preamble-order`                    | Checks that the preamble headers are in the correct order.                                                |
| `preamble-req`                      | Checks that all required preamble headers are present.                                                    |
| `preamble-req-category`             | Checks that the `category` header is present only when required.                                          |
| `preamble-req-last-call-deadline`   | Checks that the `last-call-deadline` header is present only when required.                                |
| `preamble-req-withdrawal-reason`    | Checks that the `withdrawal-reason` header is present only when required.                                 |
| `preamble-trim`                     | Checks that there is no extra whitespace around preamble fields.                                          |
| `preamble-uint-requires`            | Checks that the `requires` header is a sorted list of non-negative integers.                              |
