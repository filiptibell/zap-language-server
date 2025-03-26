; Container types
[
  (struct_type)
  (enum_type)
  (enum_variant_fields)
  (event_declaration)
  (function_declaration)
] @indent

; Opening brackets
[
  "{"
  "("
  "["
] @indent

; Closing brackets
[
  "}"
  ")"
  "]"
] @end

; Ignore comments
(comment) @ignore
