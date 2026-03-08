# LyraLang Macros — Syntax Extension (P1-026)

## Overview

The LyraLang syntax extension system provides a hygienic macro facility for
defining and expanding new surface syntax within `.lyra` source files. In
Stage 0 the macro system is recognized conservatively through two builtin
call forms over the existing parser surface.

## Surface Forms

### `syntax_define(name, template)`

Registers a macro named `name` with the given `template` string. The
`template_arity` of the definition is inferred by counting `$` hole markers
in the template string.

```lyra
let m = syntax_define("double", "syntax_expand(double, $x) => $x + $x")
```

This records a `MacroDefinition` with:
- `name = "double"`
- `template_arity = 2` (two `$` holes)
- `template_summary = "syntax_expand(double, $x) => $x + $x"`

### `syntax_expand(name, args...)`

Expands a previously defined macro. The first argument is the macro name
(as a string literal); subsequent arguments are the expansion arguments.

```lyra
let r = syntax_expand("double", 5)
```

This records a `MacroExpansion` with:
- `macro_name = "double"`
- `expansion_index = 0` (first expansion in the program)
- `argument_count = 1`
- `hygienic = true` (definition was found, arity matched)

## Hygiene Model

Each expansion introduces hygienic bindings for its arguments. Introduced
bindings are renamed by appending `#gensym{N}` where N is a monotonically
increasing counter per program:

```
original_name → original_name#gensym0
```

The top-level `hygienic` flag on the judgment is `true` iff no
`HygieneViolation` error was produced during analysis.

## Error Kinds

| Kind | Condition |
|------|-----------|
| `ParseError` | Source could not be parsed. |
| `UndefinedMacro` | `syntax_expand` refers to a name not registered by `syntax_define`. |
| `ArityMismatch` | Wrong number of arguments to `syntax_define` (not 2) or argument count mismatch at expansion site. |
| `HygieneViolation` | A macro expansion would capture an outer binding. |

## Checker API

```rust
use lyralang::macros::check;

let output = check(source);
// output.judgment — Some(SyntaxExtensionJudgment) on success
// output.errors   — Vec<SyntaxExtensionError>
```

## Stage 0 Scope

Stage 0 recognizes the two surface forms and performs:
- Macro registration and arity inference.
- Expansion lookup, arity checking, and hygienic binding generation.
- Error recovery (analysis continues after non-fatal errors).

Full template instantiation and AST rewriting are planned for later stages.
