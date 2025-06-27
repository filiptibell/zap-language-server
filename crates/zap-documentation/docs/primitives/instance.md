Roblox Instances can be passed through Zap.

# Warning

If a non-optional instance results in `nil` when received, it will cause a deserialize error and the packet will be dropped.
Instances are turned into `nil` when they don't exist on the reciever - for example: an instance from the server that isn't streamed into a client or an instance that only exists on the client.

Immediately before making important changes (such as deletion or parenting to non-replicated storage) to non-optional `Instance`s that are referenced in events fired to clients, consider calling `SendEvents` to flush the outgoing queue.
This way, even with batching, your Zap events are guaranteed to arrive before deletion replicates.

If you want to send an instance that may not exist, you must make it optional.

You can also specify what kind of instance you want to accept, for example:

```zap
type Part = Instance(BasePart)
```

Classes that inherit your specified class will be accepted, for example `Part`.
