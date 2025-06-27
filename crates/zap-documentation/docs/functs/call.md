This field determines how the function is listened to on the server. The function will take the `args` as parameters and return `rets`.

- `Async` functions can be listened to by one function, and they are called asynchronously.
- `Sync` functions can be listened to by one function, and they are called synchronously.

# Danger

Synchronous functions are not recommended, and should only be used when performance is critical.

- If a synchronous function callback yields it will cause **undefined and game-breaking behavior**.
- If a synchronous function callback errors it will cause **the packet to be dropped**.

Use synchronous functions with extreme caution.
