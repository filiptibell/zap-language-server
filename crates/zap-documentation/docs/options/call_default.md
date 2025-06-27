The default `call` field that will be used for events. See [call](events.html#call) for possible options.

### Example

```zap
opt call_default = "ManySync"
```

```zap
opt call_default = "Polling"
```

### Default

Requires an explicit `call` field defined on every event.
