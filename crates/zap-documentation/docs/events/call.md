This field determines how the event is listened to on the receiving side.

- `ManyAsync` events can be listened to by many functions, and they are called asynchronously.
- `ManySync` events can be listened to by many functions, and they are called synchronously.
- `SingleAsync` events can be listened to by one function per actor, and they are called asynchronously.
- `SingleSync` events can be listened to by one function per actor, and they are called synchronously.
- `Polling` `[0.6.18+]` events are received once per actor by iterating through `event.iter()`.

# Danger

Synchronous events are not recommended, and should only be used when performance is critical.

- If a synchronous event callback yields it will cause **undefined and game-breaking behavior**.
- If a synchronous event callback errors it will cause **the packet to be dropped**.

Use synchronous events with extreme caution.
