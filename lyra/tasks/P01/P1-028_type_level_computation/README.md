# P1-028 — Type-Level Computation

## Mission
Const generics, type families, compile-time computation with termination checking.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/TYPELEVEL.md`
- `interfaces/specs/lyralang_type_level_computation_v1.json`
- `lyralang/src/typelevel/mod.rs`
- `fixtures/lyralang/typelevel/typelevel_valid.lyra`
- `fixtures/lyralang/typelevel/typelevel_nonterminating.lyra`
- `goldens/lyralang/typelevel/typelevel_valid.json`
- `goldens/lyralang/typelevel/typelevel_nonterminating.json`

## Surface Forms (Stage 0)
- `let name: Int = literal` — const generic parameter binding.
- `const_add(a, b)` — compile-time addition.
- `const_mul(a, b)` — compile-time multiplication.
- `type_family_define(name, params, result)` — type family definition.
- `type_family_apply(name, args...)` — type family application.

## Termination Checking
- `const_add`/`const_mul` with literal arguments: `termination_reason = "constant_folding"`.
- Nested `const_add`/`const_mul` calls (bounded recursion): `termination_reason = "bounded_recursion"`.
- Arguments containing unknown variables: `NonTerminatingComputation` error.
- `type_family_apply`: `termination_reason = "structural"` when the family is known.
