# P1-032 Acceptance Criteria

## Exit Gate Requirements

All criteria must be satisfied for Phase 1 to be considered complete.

### 1. Round-trip parse/print idempotency
- [x] Parse source to AST and normalized_source
- [x] Re-parse normalized_source
- [x] Both normalized forms are identical
- [x] Verified for: integer literal, if-else expression, function call

### 2. Type soundness verification
- [x] Type checker infers types without errors for integer literals
- [x] Type checker infers types without errors for boolean literals
- [x] Type checker infers types without errors for builtin function calls (add)
- [x] Type checker infers types without errors for comparison builtins (eq)
- [x] All sources produce a ProgramJudgment

### 3. Compile hello world
- [x] `let greeting = 42` passes parse
- [x] `let greeting = 42` passes type check
- [x] `let greeting = 42` passes codegen (produces CodegenProgram)
- [x] `let greeting = 42` passes bytecode emission (produces non-empty encoded bytes)

### 4. Compile fibonacci
- [x] `let fib = add(add(1, 1), add(1, 2))` passes full pipeline
- [x] Semantic evaluation succeeds without errors

### 5. Compile symbolic expression
- [x] `let result = if eq(add(1, 2), 3) { 1 } else { 0 }` passes full pipeline
- [x] Conditional with nested function calls compiles to bytecode

### 6. Effect checking integration
- [x] Effect checker runs without errors on arithmetic expression

### 7. Pattern matching integration
- [x] Boolean match with true/false arms parses and checks
- [x] PatternProgramJudgment reports exhaustive = true

## Verification Command

```bash
cargo test --package lyralang --test language_integration
```

All 7 tests pass.
