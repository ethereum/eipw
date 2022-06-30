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
