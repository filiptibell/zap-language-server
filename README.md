# Zap Language

Basic editor support for [Zap](https://zap.redblox.dev), providing syntax highlighting, indentation, and code folding.

Grammar implementations are pulled from two separate repositories:

- VSCode uses the [TextMate](https://github.com/filiptibell/tmlanguage-zap) grammar.
- Zed uses the [tree-sitter](https://github.com/filiptibell/tree-sitter-zap) grammar.

Please report issues with syntax and/or highlighting to their corresponding repositories linked above.

## Zap Language Server

An experimental language server is also available, adding support for:

- Information on hover for keywords, primitive types, and user-defined types
- Completion for keywords, primitive types, and user-defined types
- Go to definition for user-defined types

Currently, the language server only runs in the Zed editor, and can be installed as such:

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository, and navigate to the root directory
3. Run `cargo install --path crates/zap-language-server` to compile and install the language server
4. Install the Zed extension at `editors/zed` as a [dev extension](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)
