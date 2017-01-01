# Triggerable custom compiler error

[![Crates.io](https://img.shields.io/crates/v/compiler_error.svg)](https://crates.io/crates/compiler_error)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`compiler_error` is a rust compiler plugin that simply triggers a compilation
error on demand and prints the token it receives, it is similar to the `#error`
directive in the C preprocessor.

It is quite useful to provide helpful error message in macros with non-trival syntaxes.

## Todo

- [ ] support printing optionally multiple tokens
- [ ] provide useful examples
