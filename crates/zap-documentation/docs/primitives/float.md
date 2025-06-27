Floating point numbers are numbers with a decimal point. They can be positive and negative.

Zap supports `f32` and `f64` floating point numbers, but unlike integers these numbers do not have a set range.
Instead the size determines the precision of the number. Determining what precision you need is out of scope for this documentation.

`f64`s are able to store integers accurately up to `2^53` (9,007,199,254,740,992).
This is larger than the maximum value of `u32`, but also twice the size.

It should also be noted that the type of numbers in Luau is `f64`.
