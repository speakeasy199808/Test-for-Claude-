# P1-031 — Source Map Generation

## Summary

Implements bidirectional source-map generation for LyraLang as a library
module (`lyralang::sourcemap`). Maps AST nodes to bytecode instruction
offsets and provides structured debugger hints for breakpoint sites, step
points, and scope entry/exit.

## Module

`lyralang/src/sourcemap/mod.rs`

## Public surface

| Item | Kind | Description |
|------|------|-------------|
| `SourceMapGenerator` | struct | Stateless generator |
| `SourceMapGenerator::new()` | fn | Create a generator |
| `SourceMapGenerator::generate_source(&self, source: &str) -> SourceMapOutput` | fn | Full pipeline |
| `generate(source: &str) -> SourceMapOutput` | fn | Convenience wrapper |
| `SourceMapOutput` | struct | normalized_source + source_map + errors |
| `SourceMap` | struct | Complete map: entries + index + hints |
| `SourceMapEntry` | struct | One source ↔ bytecode mapping record |
| `SourceMapEntryKind` | enum | LetBinding / Expression / FunctionCall / ControlFlow / Literal |
| `SourceMapIndex` | struct | by_bytecode_offset + by_source_line sorted vecs |
| `DebuggerHint` | struct | Hint for debugger tooling |
| `DebuggerHintKind` | enum | BreakpointSite / StepPoint / ScopeEntry / ScopeExit |
| `SourceMapError` | struct | Diagnostic with kind + message + span |
| `SourceMapErrorKind` | enum | ParseError / CodegenError / MappingInconsistency |

## Pipeline

1. `crate::parser::parse(source)` — obtain AST
2. `crate::codegen::generate(source)` — obtain IR instruction stream
3. Walk AST statements and expressions, pairing each node with the
   next available instruction index
4. Build `SourceMapIndex` by sorting `entries` by bytecode offset and
   source line respectively
5. Emit `DebuggerHint`s: `BreakpointSite` at every `let` binding,
   `StepPoint` at every expression, `ScopeEntry`/`ScopeExit` for block
   and program boundaries

## Source map format

- `source_uri`: `"source://lyra"`
- `format_version`: `"lyrasourcemap/v1"`
- All line/column numbers are 1-based (matching the lexer span convention)

## Dependencies

- `crate::parser::parse`
- `crate::codegen::generate`
- `crate::lexer::span::SourceSpan`

## Fixtures

- `fixtures/lyralang/sourcemap/sourcemap_hello.lyra`
- `fixtures/lyralang/sourcemap/sourcemap_hello.json`

## Goldens

- `goldens/lyralang/sourcemap/sourcemap_hello.json`
