Strings are defined using the word `string`. For example:

```zap
type S = string
```

The length of strings can be constrained by placing a range within parenthesis after the `string` keyword.
For example, if you wanted to constrain a string between `3` and `20` characters (like a username) you could do:

```zap
type S = string(3..20)
```
