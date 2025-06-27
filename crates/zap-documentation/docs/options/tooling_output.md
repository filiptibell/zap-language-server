This options allows you to configure where Zap will output its generated tooling code.

The path is relative to the configuration file and should point to a lua(u) file.

### Default

```zap
opt tooling_output = "./network/tooling.lua"
```

### Example

```zap
opt tooling = true
opt tooling_output = "src/ReplicatedStorage/RemoteName.profiler.luau"
```

or

```zap
opt typescript = true
opt tooling = true
opt tooling_output = "src/include/RemoteName.profiler.lua"
```
