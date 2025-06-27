This option determines if Zap automatically sends reliable events and functions each Heartbeat.

When enabled, the `SendEvents` function exported from the client and server modules that must be called manually.

This is useful when you can easily run `SendEvents` after all events have been fired each frame.

# Danger

At the time of writing (January 2024), Roblox has an issue where firing remotes at too high of a rate (above 60 hz) can cause the server to have incredibly high network response times.

**This causes servers to essentially crash, and all clients to disconnect.**

This can be mitigated by firing remotes to the server at a timed rate, so as to not exceed 60 hz.

```luau
local Timer = 0

RunService.Heartbeat:Connect(function(DeltaTime)
	Timer += DeltaTime

	-- Only send events 60 times per second
	if Timer >= 1 / 60 then
		Timer = 0
		Zap.SendEvents()
	end
end)
```

Note that Zap uses `RunService.Heartbeat` and a 61 hz rate by default.

### Default

`false`

### Options

- `true`
- `false`

### Example

```zap
opt manual_event_loop = true
```
