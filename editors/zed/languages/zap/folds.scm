; Allow folding of all brace-enclosed blocks
[
  (struct_type)
  (enum_type)
  (event_declaration)
  (function_declaration)
] @fold

; Allow folding of comma-separated lists that may span multiple lines
(tuple) @fold
