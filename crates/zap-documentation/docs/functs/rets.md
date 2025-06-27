This field determines the data that is sent back to the client from the server. It can be any Zap type.

- If the server doesn't return any data, the `rets` field should be excluded.
- Unlike `args`, `rets` cannot be named.
- The function can return multiple values by separating each type with a comma, and wrapping them all in parentheses.
