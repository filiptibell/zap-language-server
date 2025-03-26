; Type declarations
(type_declaration
  name: (identifier) @name) @item

; Event declarations
(event_declaration
  name: (identifier) @name) @item

; Function declarations
(function_declaration
  name: (identifier) @name) @item

; Comments above declarations as annotations
(comment) @annotation
