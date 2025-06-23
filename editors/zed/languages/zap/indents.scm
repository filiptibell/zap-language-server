; Struct indentation
(struct_type "{" "}" @end) @indent

; Enum indentation
(enum_type "{" "}" @end) @indent
(enum_variant "{" "}" @end) @indent

; Declarations indentation
(event_declaration "{" "}" @end) @indent
(function_declaration "{" "}" @end) @indent
(namespace_declaration "{" "}" @end) @indent

; General bracket indentation
(_ "[" "]" @end) @indent
(_ "(" ")" @end) @indent
(_ "{" "}" @end) @indent
