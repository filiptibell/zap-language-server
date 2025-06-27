Types are named definitions that allow you to create reusable data structures in your Zap configuration.
The `type` keyword lets you define custom types using any of Zap's type constructors such as struct, enum, map, etc.

## Defining Types

Types are defined using the `type` keyword followed by a name, an equals sign, and the type definition.

```zap
type PlayerData = struct {
    Name: string,
    Score: u32,
}
```

## Referencing Types

Once defined, types can be referenced by name throughout your Zap configuration.
This allows you to use the same data structures in multiple events or functions, create more complex
types by composing simpler ones, and helps maintain consistency across your entire Zap configuration.

```zap
type Inventory = struct {
    Items: string[],
    Gold: u32,
}

event InventoryUpdated = {
    from: Server,
    type: Reliable,
    call: ManyAsync,
    data: Inventory,
}

funct GetInventory = {
    call: Async,
    rets: Inventory,
}
```

Custom types are accessible throughout your entire Zap configuration file, regardless of where they are defined.
