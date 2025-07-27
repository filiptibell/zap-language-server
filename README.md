<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<div align="center">
  <a href="https://github.com/filiptibell/zap-language/actions">
  <img src="https://shields.io/endpoint?url=https://badges.readysetplay.io/workflow/filiptibell/zap-language/ci.yaml" alt="CI status" />
  </a>
  <a href="https://github.com/filiptibell/zap-language/actions">
    <img src="https://shields.io/endpoint?url=https://badges.readysetplay.io/workflow/filiptibell/zap-language/release.yaml" alt="Release status" />
  </a>
  <a href="https://github.com/filiptibell/zap-language/blob/main/LICENSE.txt">
    <img src="https://img.shields.io/github/license/filiptibell/zap-language.svg?label=License&color=informational" alt="License" />
  </a>
</div>

<br/>

# Zap Language Server

Full editor support for [Zap](https://zap.redblox.dev), providing syntax highlighting, indentation, and code folding.

Also supports the following features using the [LSP](https://microsoft.github.io/language-server-protocol/) (Language Server Protocol):

- 🧐 Diagnostics for all the same errors, warnings, and hints as the Zap CLI
- 🔮 Information on hover for keywords, primitive types, and user-defined types
- 🪄 Completion for keywords, primitive types, and user-defined types
- 🎯 Go to definition & renaming for user-defined types
- 📝 Full document auto-formatting

## Installation

Zap Language Server can be installed as an extension from:

- The [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=filiptibell.zap-language-server) (VSCode)
- The [Open VSX Registry](https://open-vsx.org/extension/filiptibell/zap-language-server) (Cursor, Windsurf, ...)
- The [Zed Extensions](https://zed.dev/extensions) registry

Extensions can also be built and installed from source:

<details>
<summary> Manual Installation - VSCode </summary>

1. [Install Bun](https://bun.sh/docs/installation)
2. Clone this repository, and navigate to the `editors/vscode` directory
3. Build and install the extension by running these three commands, in order:
    ```bash
    bun install
    bun pm trust --all
    bun run extension-install
    ```

</details>

<details>
<summary> Manual Installation - Zed </summary>

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository, and navigate to the root directory
3. Install the Zed extension at `editors/zed` as a [dev extension](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)

</details>

## CLI

The language server can also be used as a standalone tool, and includes a CLI for formatting Zap files.

It can be installed from the [latest release](https://github.com/filiptibell/zap-language-server/releases/latest) using something like [Rokit](https://github.com/rojo-rbx/rokit):

```bash
rokit add filiptibell/zap-language-server
```

Once installed, the CLI is very similar to [StyLua](https://github.com/JohnnyMorganz/StyLua) and/or [prettier](https://prettier.io/).
The formatter is intentionally opinionated and does not have any configuration.

### Formatter

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

### Server

```bash
# Starts the language server using the default transport (stdio)
zap-language-server serve

# Starts the language server using the TCP transport and the given port
zap-language-server serve --port <port_number>
```

## Reporting Bugs

Grammar implementations are pulled from two separate repositories:

- VSCode uses the [TextMate](https://github.com/filiptibell/tmlanguage-zap) grammar.
- Zed uses the [tree-sitter](https://github.com/filiptibell/tree-sitter-zap) grammar.

Please report issues with syntax and/or highlighting to their corresponding linked repositories.
