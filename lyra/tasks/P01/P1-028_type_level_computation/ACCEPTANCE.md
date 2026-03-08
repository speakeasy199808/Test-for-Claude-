# Acceptance — P1-028 Type-Level Computation

## Acceptance Criteria

1. Stage 0 exposes `const_add`, `const_mul`, `type_family_define`, and `type_family_apply`.
2. `let name = literal_int` bindings are recorded as `ConstGeneric` with `value_type = "Int"`.
3. `const_add(a, b)` and `const_mul(a, b)` with literal arguments produce `CompileTimeComputation`
   with `terminates = true` and `termination_reason = "constant_folding"`.
4. `const_add`/`const_mul` with variable arguments produce `NonTerminatingComputation` errors
   and `terminates = false`.
5. `type_family_define(name, params, result)` records a `TypeFamily`.
6. `type_family_apply(name, args...)` produces a `CompileTimeComputation` with `op = "type_apply"`.
7. Application of an undefined type family produces an `UndefinedTypeFamily` error.
8. `all_terminate = true` iff no `NonTerminatingComputation` errors occurred.
9. Fixtures and goldens cover both success and error behavior.

## Verification Method
- Review `docs/lyralang/TYPELEVEL.md` and `interfaces/specs/lyralang_type_level_computation_v1.json`.
- Inspect `lyralang/src/typelevel/mod.rs` implementation.
- Run checker over `fixtures/lyralang/typelevel/typelevel_valid.lyra` and compare to golden.
- Run checker over `fixtures/lyralang/typelevel/typelevel_nonterminating.lyra` and compare to golden.

## Evidence Required
- `docs/lyralang/TYPELEVEL.md`
- `interfaces/specs/lyralang_type_level_computation_v1.json`
- `lyralang/src/typelevel/mod.rs`
- `fixtures/lyralang/typelevel/typelevel_valid.lyra`
- `fixtures/lyralang/typelevel/typelevel_nonterminating.lyra`
- `goldens/lyralang/typelevel/typelevel_valid.json`
- `goldens/lyralang/typelevel/typelevel_nonterminating.json`
