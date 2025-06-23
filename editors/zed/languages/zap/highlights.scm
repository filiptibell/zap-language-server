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
  "namespace"
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
  "."
  ".."
] @punctuation.delimiter

; Constants & Builtins
(event_from_value) @constant  ; Server/Client
(event_type_value) @constant  ; Reliable/Unreliable
(event_call_value) @constant  ; ManyAsync/ManySync/etc
(function_call_value) @constant  ; Async/Sync

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
  "Instance"
] @type.builtin

; Options and Variables
(option_declaration
(identifier) @variable)
(identifier) @variable

; Declarations
(type_declaration
  name: (identifier) @type)

(event_declaration
  name: (identifier) @function)

(function_declaration
  name: (identifier) @function)

(namespace_declaration
  name: (identifier) @namespace)

(enum_variant
  (identifier) @enum.variant)

; Fields and Properties
(property
  name: (identifier) @property)

(tuple_value
  name: (identifier) @property)

(event_from_field "from" @property)
(event_type_field "type" @property)
(event_call_field "call" @property)
(event_data_field "data" @property)

(function_call_field "call" @property)
(function_args_field "args" @property)
(function_rets_field "rets" @property)

; Literals
["true" "false"] @boolean
(boolean) @boolean
(number) @number
(string) @string

; Comments
(comment) @comment
(doc_comment) @comment.doc

; Ranges
(range
  (range_exact) @operator)
(range
  (range_inexact) @operator)
(array
  (array_exact) @operator)
(array
  (array_inexact) @operator)

; Type modifiers
(optional_type
  "?" @operator)

(namespaced_type
  namespace: (identifier) @namespace)
