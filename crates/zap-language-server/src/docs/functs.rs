pub const FUNCT_FIELD_DESCRIPTION_CALL: &str = "
This field determines how the function is listened to on the server. The function will take the `args` as parameters and return `rets`.

- `Async` functions can be listened to by one function, and they are called asynchronously.
- `Sync` functions can be listened to by one function, and they are called synchronously.

# Danger

Synchronous functions are not recommended, and should only be used when performance is critical.

- If a synchronous function callback yields it will cause **undefined and game-breaking behavior**.
- If a synchronous function callback errors it will cause **the packet to be dropped**.

Use synchronous functions with extreme caution.
";

pub const FUNCT_FIELD_DESCRIPTION_ARGS: &str = "
This field determines the data that is sent to the server. It can be any Zap type.

- If the client doesn't send any data, the `args` field should be excluded.
- Parameter names and parentheses are optional to preserve backwards compatibility. If parantheses are excluded, the function can only have one unnamed parameter.
";

pub const FUNCT_FIELD_DESCRIPTION_RETS: &str = "
This field determines the data that is sent back to the client from the server. It can be any Zap type.

- If the server doesn't return any data, the `rets` field should be excluded.
- Unlike `args`, `rets` cannot be named.
- The function can return multiple values by separating each type with a comma, and wrapping them all in parentheses.
";
