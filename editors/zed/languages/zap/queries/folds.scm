; Allow folding of all brace-enclosed blocks
[
  (struct_type)
  (enum_type)
  (enum_variant_fields)
  (event_declaration)
  (function_declaration)
] @fold

; Allow folding of comma-separated lists that span multiple lines
(event_data_tuple) @fold
