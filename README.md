# Zap Language Server

Basic editor support for [Zap](https://zap.redblox.dev), providing syntax highlighting, indentation, and code folding.

Grammar implementations are pulled from two separate repositories:

- VSCode uses the [TextMate](https://github.com/filiptibell/tmlanguage-zap) grammar.
- Zed uses the [tree-sitter](https://github.com/filiptibell/tree-sitter-zap) grammar.

Please report issues with syntax and/or highlighting to their corresponding repositories linked above.

## Formatter

The language server can also be used as a standalone tool, which includes a formatter.
It can be installed from the [latest release](https://github.com/filiptibell/zap-language-server/releases/latest) using something like [Rokit](https://github.com/rojo-rbx/rokit).

```bash
rokit add filiptibell/zap-language-server
```

Once installed, the CLI is very similar to [StyLua](https://github.com/JohnnyMorganz/StyLua) and/or [prettier](https://prettier.io/).
The formatter intentionally does not have any extra configuration and is opinionated.

```bash
# Format the specified file, writing results to standard
# output (stdin can be used instead of a file using '-')
zap-language-server fmt <file_path>

# Format the specified file, overwriting it
zap-language-server fmt <file_path> --write

# Check if formatting would change the file contents,
# outputting a diff if it would change - does not write
zap-language-server fmt <file_path> --check
```

## Language Server

An experimental language server is available, adding support for:

- Information on hover for keywords, primitive types, and user-defined types
- Completion for keywords, primitive types, and user-defined types
- Go to definition for user-defined types
- Full document auto-formatting

Currently, the language server only runs in the Zed editor, and can be installed as such:

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository, and navigate to the root directory
3. Run `cargo install --path crates/zap-language-server` to compile and install the language server
4. Install the Zed extension at `editors/zed` as a [dev extension](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)
