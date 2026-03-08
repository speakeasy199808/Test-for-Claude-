# LyraLang Stage 0 Code Generation

## Purpose

P1-019 lowers the currently executable Stage 0 AST into a deterministic
register-VM intermediate representation. Byte-level encoding is deferred to
P1-020; this task defines the canonical pre-serialization IR.

## Format Version

The canonical Stage 0 format tag is:

```text
lyravm-stage0-ir-v1
```

## Determinism Requirements

The seed code generator must:

- allocate registers in stable creation order
- allocate labels in stable traversal order
- preserve source-ordered statement lowering
- render instructions canonically
- reject unsupported constructs with stable diagnostics

## Register Model

The Stage 0 IR is a flat virtual register machine.

- registers are named `r0`, `r1`, `r2`, ...
- labels are named `L0`, `L1`, `L2`, ...
- the final program result is identified by a single entry register

## Lowered Instruction Families

The seed implementation emits canonical renderings for:

- constants: `const.unit`, `const.int`, `const.bool`
- data movement: `move`
- self reference: `selfref @current_program`, `selfref @current_receipt`, `selfref @ledger_state`
- integer prefix arithmetic: `neg.int`
- binary operators: `add.int`, `sub.int`, `mul.int`, `div.int`, `mod.int`, comparisons, and boolean conjunction/disjunction
- branching: `branch.if`, `jump`, `label`
- builtins: `call name(...)`
- pattern assertions: `assert.int`, `assert.bool`
- terminal failure for unsupported non-exhaustive match fallthrough: `trap`

## Boundary of This Task

P1-019 generates deterministic IR only.
P1-020 will define the canonical bytecode object format and serialization.
