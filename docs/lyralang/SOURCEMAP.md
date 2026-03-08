# LyraLang Source Map Generation (P1-031)

## Overview

The `lyralang::sourcemap` module generates bidirectional mappings between source
locations and bytecode instruction indices. The output is used by debugger
tooling to support breakpoints, single-stepping, and scope display.

## Pipeline

```
source
  │
  ├─► parser::parse    → Program AST  (spans: line/column/byte offset)
  │
  └─► codegen::generate → CodegenProgram (instructions: Vec<String>)
            │
            └─► SourceMapGenerator
                  │
                  ├─► Walk AST statements & expressions
                  │     ↳ pair each node with instructions[offset]
                  │
                  ├─► Build SourceMapIndex
                  │     ↳ by_bytecode_offset (sorted)
                  │     ↳ by_source_line     (sorted)
                  │
                  └─► Emit DebuggerHints
                        ↳ BreakpointSite at every let binding
                        ↳ StepPoint at every expression
                        ↳ ScopeEntry / ScopeExit for program and block boundaries
```

## Format

| Field | Value |
|-------|-------|
| `source_uri` | `"source://lyra"` |
| `format_version` | `"lyrasourcemap/v1"` |
| Line/column numbering | 1-based (matches lexer `SourceSpan`) |
| Byte offsets | Byte offsets into the normalized source string |

## Quick start

```rust
use lyralang::sourcemap;

let output = sourcemap::generate("let x = 42\nx");
let map = output.source_map.unwrap();

// Forward lookup: source line 1 → bytecode offset
let entry = map.entries.iter().find(|e| e.source_line == 1).unwrap();
println!("bytecode offset: {}", entry.bytecode_offset);

// Reverse lookup: bytecode offset 0 → entries index
let (_, idx) = map.index.by_bytecode_offset[0];
println!("source line: {}", map.entries[idx].source_line);
```

## SourceMapEntry fields

| Field | Type | Description |
|-------|------|-------------|
| `source_line` | `u32` | 1-based line number |
| `source_column` | `u32` | 1-based column number |
| `source_span_start` | `usize` | Byte offset (inclusive) |
| `source_span_end` | `usize` | Byte offset (exclusive) |
| `bytecode_offset` | `u32` | IR instruction index |
| `ir_instruction` | `String` | Canonical IR text |
| `entry_kind` | `SourceMapEntryKind` | Semantic category |

## SourceMapEntryKind

| Variant | Trigger |
|---------|---------|
| `LetBinding` | `let` statement |
| `Expression` | General expression |
| `FunctionCall` | Call expression |
| `ControlFlow` | `if` or `match` expression |
| `Literal` | Integer, boolean, or string literal |

## DebuggerHintKind

| Variant | Meaning |
|---------|---------|
| `BreakpointSite` | A valid breakpoint location (every `let` binding) |
| `StepPoint` | A single-step point (every expression) |
| `ScopeEntry` | Entering a lexical scope |
| `ScopeExit` | Exiting a lexical scope |

## Error handling

If parsing fails, `SourceMapOutput.source_map` is `None` and
`SourceMapOutput.errors` contains `SourceMapErrorKind::ParseError` entries.
If code generation fails, errors have kind `CodegenError`.

## Types reference

See `interfaces/specs/lyralang_sourcemap_v1.json`.

## Fixtures

- `fixtures/lyralang/sourcemap/sourcemap_hello.lyra` — sample source
- `fixtures/lyralang/sourcemap/sourcemap_hello.json` — expected source map

## Goldens

- `goldens/lyralang/sourcemap/sourcemap_hello.json`
