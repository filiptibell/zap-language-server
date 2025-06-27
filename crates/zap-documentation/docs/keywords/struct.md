Structs are similar to Interfaces, and are a collection of statically named fields with different types.

To define a struct, use the `struct` keyword followed by a Luau interface-like syntax.
For example, a struct representing an item in a shop would look like:

```zap
type Item = struct {
	name: string,
	price: u16,
}
```
