# markdown-no-backticks

This lint ensures that EIP references (e.g., EIP-1234) are not wrapped in backticks. This helps maintain consistent formatting across all EIPs and follows the established community convention.

## Examples

### ❌ Invalid: Using backticks around EIP references

```markdown
This proposal builds on `EIP-1234` and `EIP-5678`.
```

### ✅ Valid: EIP references without backticks

```markdown
This proposal builds on EIP-1234 and EIP-5678.
```

## Behavior in Different Contexts

This lint enforces consistent behavior across all contexts. EIP references should never use backticks, regardless of where they appear:

1. Regular Text:
   ```markdown
   // ❌ Incorrect
   This implements `EIP-1234`.
   
   // ✅ Correct
   This implements EIP-1234.
   ```

2. Image Alt Text:
   ```markdown
   // ❌ Incorrect
   ![Diagram of `EIP-1234` flow](diagram.png)
   
   // ✅ Correct
   ![Diagram of EIP-1234 flow](diagram.png)
   ```

3. Self References:
   ```markdown
   // ❌ Incorrect
   This is `EIP-1234`, which defines...
   
   // ✅ Correct
   This is EIP-1234, which defines...
   ```

4. Technical Sections:
   ```markdown
   // ❌ Incorrect
   If `salt` is not provided, this EIP uses `EIP-6051` as default.
   
   // ✅ Correct
   If `salt` is not provided, this EIP uses EIP-6051 as default.
   ```

## Special Cases

1. Code Blocks: EIP references inside code blocks (``` ```) are allowed to use backticks, as they may be part of code examples:
   ```solidity
   // This is fine
   function implementEIP1234() {
       // EIP-1234 implementation
   }
   ```

2. Other Code References: Other technical references like function names, variables, or contract names should still use backticks:
   ```markdown
   The function `implementERC20()` follows EIP-20.
   ```

## Rationale

EIP references are meant to be read as part of the text flow and are not code snippets. Using backticks around them:
1. Disrupts the reading flow
2. Incorrectly suggests they are code elements
3. Creates inconsistency in how EIPs are referenced across the documentation

This lint enforces the established community convention of treating EIP references as natural parts of the text, not as code elements. This convention is widely followed across the Ethereum ecosystem and helps maintain consistency in EIP documentation.

## Configuration

This lint uses a regular expression pattern to identify EIP references. By default, it matches the pattern `EIP-[0-9]+`.
