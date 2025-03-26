; Keywords
[
  "type"
  "opt"
  "event"
  "funct"
  "struct"
  "enum"
  "set"
  "map"
] @keyword

; Operators
"=" @operator
"?" @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ":"
  ","
  ".."
] @punctuation.delimiter

; Constants & Builtins
(event_from_value) @constant  ; Server/Client
(event_type_value) @constant  ; Reliable/Unreliable
(event_call_value) @constant  ; ManyAsync/ManySync/etc
(function_call_value) @constant  ; Async/Sync

[
  "true"
  "false"
] @boolean

; Types
[
  "string"
  "boolean"
  "f64"
  "f32"
  "u8"
  "u16"
  "u32"
  "i8"
  "i16"
  "i32"
  "CFrame"
  "AlignedCFrame"
  "Vector3"
  "Vector2"
  "DateTime"
  "DateTimeMillis"
  "Color3"
  "BrickColor"
] @type.builtin

; Type declarations
(type_declaration
  name: (identifier) @type)

; Fields and Properties
(struct_field
  name: (identifier) @property)

(event_data_tuple
  name: (identifier) @property)

(enum_variant
  name: (identifier) @type)

; Options
(option_declaration
  (option_name) @constant.builtin)

; Variables
(identifier) @variable

; Literals
(number) @number
(string) @string

; Comments
(comment) @comment

; Special syntax
(range
  (inclusive_range) @operator)
(range
  (exclusive_range) @operator)
(range
  (exact_range) @operator)

; Optional types
(optional_type
  "?" @operator)
