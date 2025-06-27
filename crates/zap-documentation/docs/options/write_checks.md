This option determines if Zap should check types when writing data to the network. This is useful for development and debugging, but can see some performance hits, and should be disabled in production.

# Danger

Zap only checks types that cannot be statically checked by Luau or TypeScript.

For example, Zap will not check if a `string (20)` is a string, but it will check that the string is 20 characters long.

### Default

`true`

### Options

- `true`
- `false`

### Example

```zap
opt write_checks = true
```
