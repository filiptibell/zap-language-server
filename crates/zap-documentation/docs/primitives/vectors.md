Zap supports `vector`s with any numeric components other than `f64`.
The Z component is optional, and will result in a `0` if omitted.

```zap
type Position = vector(f32, f32, f32)
type Size = vector(u8, f32)
```

Omitting all components will emit `vector(f32, f32, f32)`.

```zap
type Position = vector
```

Zap also supports serializing `Vector3`, and to not serialise the `Z` property of a `Vector3`, you can use the `Vector2` type.
