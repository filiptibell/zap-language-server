In a large complex game, namespaces can be used to organize types, events, and functions into logical groups.
Namespaces are defined using the `namespace` keyword, and look like this:

```zap
namespace MyNamespace = {
	type Foo = u8

	event FooHappened = {
		from: Server,
		type: Reliable,
		call: SingleAsync,
		data: Foo,
	}
}

-- Types from a namespace may be referenced as such
type ReferencedFoo = MyNamespace.Foo
```
