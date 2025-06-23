(type_declaration
  "type" @context
  name: (identifier) @name) @item

(event_declaration
  "event" @context
  name: (identifier) @name) @item

(function_declaration
  "funct" @context
  name: (identifier) @name) @item

(namespace_declaration
  "namespace" @context
  name: (identifier) @name) @item

(doc_comment) @annotation
