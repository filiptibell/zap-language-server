; Struct indentation
(struct_type "{" "}" @end) @indent

; Enum indentation
(enum_unit_type "{" "}" @end) @indent
(enum_tagged_type "{" "}" @end) @indent
(enum_variant_fields "{" "}" @end) @indent

; Event/Function indentation
(event_declaration "{" "}" @end) @indent
(function_declaration "{" "}" @end) @indent

; General bracket indentation
(_ "[" "]" @end) @indent
(_ "(" ")" @end) @indent
(_ "{" "}" @end) @indent
