# P1-031 Acceptance Criteria

## Functional

- [ ] `generate("let x = 42\nx")` returns a `SourceMapOutput` with `source_map = Some(_)` and empty errors
- [ ] The resulting `SourceMap::format_version` equals `"lyrasourcemap/v1"`
- [ ] The resulting `SourceMap::source_uri` equals `"source://lyra"`
- [ ] Each `let` binding produces a `SourceMapEntry` with `entry_kind = LetBinding`
- [ ] Each `let` binding produces a `DebuggerHint` with `kind = BreakpointSite`
- [ ] The first entry produces a `DebuggerHint` with `kind = ScopeEntry` for the program boundary
- [ ] The last entry produces a `DebuggerHint` with `kind = ScopeExit` for the program boundary
- [ ] `SourceMapIndex::by_bytecode_offset` is sorted ascending by bytecode offset
- [ ] `SourceMapIndex::by_source_line` is sorted ascending by source line
- [ ] Parse errors produce `SourceMapOutput { source_map: None, errors: [..] }`
- [ ] Codegen errors produce `SourceMapOutput { source_map: None, errors: [..] }`

## Entry field requirements

- [ ] `source_line` is 1-based
- [ ] `source_column` is 1-based
- [ ] `source_span_start` is a valid byte offset into the normalized source
- [ ] `bytecode_offset` is a valid index into the IR instruction stream

## Non-functional

- [ ] All public items have `///` doc comments
- [ ] All public types derive `Clone, Debug, Serialize, Deserialize`
- [ ] Module builds without warnings under `#![deny(missing_docs)]`

## Fixtures match

- [ ] `fixtures/lyralang/sourcemap/sourcemap_hello.json` matches golden at `goldens/lyralang/sourcemap/sourcemap_hello.json`
- [ ] `fixtures/lyralang/sourcemap/sourcemap_hello.lyra` is valid LyraLang Stage 0 source
