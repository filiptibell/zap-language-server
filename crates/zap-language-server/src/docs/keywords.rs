pub const KEYWORD_EVENT_DESCRIPTION: &str = "
Events are the primary method of communication between the client and the server.
Events are also what is exposed to the developer from Zap's generated API.

## Defining Events

Events are defined using the `event` keyword.
";

pub const KEYWORD_FUNCT_DESCRIPTION: &str = "
Functions are a method of communication where the client can send arguments and have them returned by the server.
For security reasons, Zap only supports `Client -> Server -> Client` functions, not `Server -> Client -> Server`.

## Defining Functions

Functions are defined using the `funct` keyword.
";

pub const KEYWORD_STRUCT_DESCRIPTION: &str = "
Structs are similar to Interfaces, and are a collection of statically named fields with different types.

To define a struct, use the `struct` keyword followed by a Luau interface-like syntax.
For example, a struct representing an item in a shop would look like:

```zap
type Item = struct {
	name: string,
	price: u16,
}
```
";

pub const KEYWORD_ENUM_DESCRIPTION: &str = "
Zap has two types of enums, unit enums and tagged enums.

### Unit Enums

Unit enums are used to represent a set of possible values.
They are defined using the `enum` keyword, followed by a set of possible string values.
For example, a unit enum representing the status of a round would look like:

```zap
type RoundStatus = enum { Starting, Playing, Intermission }
```

This code would then create the Luau type:

```luau
type RoundStatus = \"Starting\" | \"Playing\" | \"Intermission\"
```

### Tagged Enums

Tagged enums will be very familiar to Rust users.

Tagged enums are a set of possible variants, each with attached data.
They are defined using the `enum` keyword, followed by a string which is the tag field name.
After the tag field name, a set of variants are defined.
Each variant is defined by a string tag, followed by a struct.
Variants must be separated by a comma.
Trailing commas are allowed.

```zap
type T = enum \"Type\" {
	Number {
		Value: f64,
	},
	String {
		Value: string,
	},
	Boolean {
		Value: boolean,
	},
}
```

This code would create the Luau type:

```luau
type T = { Type: \"Number\",  Value: number  }
       | { Type: \"String\",  Value: string  }
       | { Type: \"Boolean\", Value: boolean }
```

Tagged enums allow you to pass different data depending on a variant.
They are extremely powerful and can be used to represent many different types of data.
";

pub const KEYWORD_MAP_DESCRIPTION: &str = "
Maps are objects that have keys of one type, and values of another type.

Maps are defined using the `map` keyword, followed by a Luau-like map syntax.
For example, a map of `string` keys and `u8` values would look like:

```zap
type MyMap = map { [string]: u8 }
```
";

pub const KEYWORD_SET_DESCRIPTION: &str = "
Sets are equivalent to a map where all values are `true`, and are defined using the `set` keyword.
For example, a map of `string` keys to `true` would look like:

```zap
type MySet = set { string }
```
";
