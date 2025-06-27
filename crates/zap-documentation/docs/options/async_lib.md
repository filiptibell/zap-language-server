# Information

This option is not required when `yield_type` is set to `yield`

# Warning

When using `typescript`, provide the path to the RuntimeLib.

This option provides the async library to Zap. The option must include a `require` statement, as it will be fed directly into the Luau code.

When using Futures, you must provide a path to [Future by red-blox](https://github.com/red-blox/Util/tree/main/libs/Future). As of writing, there are no other future libraries for Roblox.

Zap is also compatible with almost any Promise library. Some common examples are:

- [Promise by evaera](https://github.com/evaera/roblox-lua-promise/)\*
- [Promise by Quenty](https://github.com/Quenty/NevermoreEngine/tree/main/src/promise)
- [Promise by red-blox](https://github.com/red-blox/Util/tree/main/libs/Promise)

<sup>\*The default in roblox-ts.</sup>

### Default

The path is empty.

### Example

```zap
opt yield_type = "promise"
opt async_lib = "require(game:GetService('ReplicatedStorage').Promise)"
```
