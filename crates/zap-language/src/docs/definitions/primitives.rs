pub const PRIMITIVE_DESCRIPTION_BOOLEAN: &str = "
Strings are defined using the word `boolean`. For example:

```zap
type B = boolean
```

Booleans do not have any special behavior and are simply true or false.
";

pub const PRIMITIVE_DESCRIPTION_STRING: &str = "
Strings are defined using the word `string`. For example:

```zap
type S = string
```

The length of strings can be constrained by placing a range within parenthesis after the `string` keyword.
For example, if you wanted to constrain a string between `3` and `20` characters (like a username) you could do:

```zap
type S = string(3..20)
```
";

pub const PRIMITIVE_DESCRIPTION_FLOAT: &str = "
Floating point numbers are numbers with a decimal point. They can be positive and negative.

Zap supports `f32` and `f64` floating point numbers, but unlike integers these numbers do not have a set range.
Instead the size determines the precision of the number. Determining what precision you need is out of scope for this documentation.

`f64`s are able to store integers accurately up to `2^53` (9,007,199,254,740,992).
This is larger than the maximum value of `u32`, but also twice the size.

It should also be noted that the type of numbers in Luau is `f64`.
";

pub const PRIMITIVE_DESCRIPTION_UNSIGNED: &str = "
Unsigned integers are positive integers.

| Type  | Min Value | Max Value     |
| ----- | --------- | ------------- |
| `u8`  | 0         | 255           |
| `u16` | 0         | 65,535        |
| `u32` | 0         | 4,294,967,295 |
";

pub const PRIMITIVE_DESCRIPTION_SIGNED: &str = "
Signed integers are standard integers.
They can be positive and negative.

| Type  | Min Value      | Max Value     |
| ----- | -------------- | ------------- |
| `i8`  | -128           | 127           |
| `i16` | -32,768        | 32,767        |
| `i32` | -2,147,483,648 | 2,147,483,647 |
";

pub const PRIMITIVE_DESCRIPTION_CFRAME: &str = "
Zap supports sending CFrames. There are two types of CFrame you may send - a regular `CFrame`, and an `AlignedCFrame`.

CFrame rotation matrices are compressed using the axis-angle representation.

# Serialization Behavior

CFrames are orthonormalized when sent. If you need to send a CFrame that is not orthogonal, meaning one that does not have a valid rotation matrix, it is recommended to send the components and reconstruct it on the other side.
Note that use cases for this are exceedingly rare and you most likely will not have to worry about this, as the common CFrame constructors only return orthogonal CFrames.

### Aligned CFrames

When you know that a CFrame is going to be axis-aligned, it is preferrable to use the `AlignedCFrame` type.

It uses much less bandwidth, as the rotation can just be represented as a single byte Enum of the possible axis aligned rotations.

You can think of an axis-aligned CFrame as one whose LookVector, UpVector, and RightVector all align with the world axes in some way. Even if the RightVector is facing upwards, for example, it would still be axis-aligned.

Position does not matter at all, only the rotation.

# Warning

If the CFrame is not axis aligned then Zap will throw an error, so make sure to use this type carefully! Don't let this dissuade you from using it though, as the bandwidth savings can be significant.

Here are some examples of axis-aligned CFrames.

```luau
local CFrameSpecialCases = {
	CFrame.Angles(0, 0, 0),
	CFrame.Angles(0, math.rad(180), math.rad(90)),
	CFrame.Angles(0, math.rad(-90), 0),
	CFrame.Angles(math.rad(90), math.rad(-90), 0),
	CFrame.Angles(0, math.rad(90), math.rad(180)),
	-- and so on. there are 24 of these in total.
}
```
";

pub const PRIMITIVE_DESCRIPTION_VECTOR: &str = "
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
";

pub const PRIMITIVE_DESCRIPTION_DATETIME: &str = "
Zap supports sending DateTimes.
There are two types of DateTime you may send - a regular `DateTime`, and `DateTimeMillis`.

DateTime sends the UnixTimestamp property of the DateTime object, and DateTimeMillis sends the UnixTimestampMillis property of the DateTime object.
";

pub const PRIMITIVE_DESCRIPTION_COLORS: &str = "
Zap supports sending Colors.
There are two types of Color you may send - a regular `Color3`, and `BrickColor`.

Color3 sends the RGB values of the Color3 object, and BrickColor sends the ID of the BrickColor.
";

pub const PRIMITIVE_DESCRIPTION_INSTANCE: &str = "
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
";
