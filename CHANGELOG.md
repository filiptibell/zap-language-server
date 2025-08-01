<!-- markdownlint-disable MD023 -->
<!-- markdownlint-disable MD033 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## `1.0.0` - July 27th, 2025

### Added

- Added diagnostics using the official Zap library (version `0.6.25`)

## `0.2.3` - July 26th, 2025

### Added

- Added support for completions, hovers, and formatting for the following new syntax:
    - `string.utf8`
    - `string.binary`
    - `Instance.ClassName`

## `0.2.2` - June 27th, 2025

### Added

- Added basic support for completions of property names such as `from`, `type`, `data`,
  and all other valid property names, inside of event and function declarations

### Changed

- Completions now trigger in more relevant locations
- Completions no longer always need a first character to be typed to trigger them

## `0.2.1` - June 25th, 2025

### Added

- Added full support for namespaces, meaning they now support:
    - Completions
    - Renaming
    - Go-to definition
    - Finding references

## `0.2.0` - June 24th, 2025

### Added

- Added basic support for namespaces
- Added full support for renaming user-defined types
- Added full support for go-to definition for user-defined types
- Added full support for finding all references for user-defined types

## `0.1.0` - May 23rd, 2025

### Added

- Added support for formatting tuples

### Changed

- Empty enum variants now format on a single line

### Fixed

- Fixed some parsing issues with tuples

[#2]: https://github.com/rojo-rbx/rokit/pull/2

## `0.0.0` - May 21st, 2025

Initial testing release
