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
type RoundStatus = "Starting" | "Playing" | "Intermission"
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
type T = enum "Type" {
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
type T = { Type: "Number",  Value: number  }
       | { Type: "String",  Value: string  }
       | { Type: "Boolean", Value: boolean }
```

Tagged enums allow you to pass different data depending on a variant.
They are extremely powerful and can be used to represent many different types of data.
