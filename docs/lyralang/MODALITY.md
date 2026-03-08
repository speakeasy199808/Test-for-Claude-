# LyraLang Stage 0 Modal Types

## Purpose

Stage 0 modal typing lets the compiler distinguish between kinds of knowledge,
not just shapes of values. The kernel introduces five epistemic modalities:

- `Fact[T]`
- `Hypothesis[T]`
- `Unknown[T]`
- `Necessary[T]`
- `Possible[T]`

These are explicit internal types in the shared kernel. They are deterministic,
canonical, and serializable, so later proof, receipt, and audit phases can rely
on them without reinterpretation.

## Canonical Forms

Modal types serialize canonically as:

- `Fact[Int]`
- `Hypothesis[Bool]`
- `Unknown[Capability]`
- `Necessary[Product[Int, Bool]]`
- `Possible[Socket]`

Evidence token types serialize canonically as:

- `Evidence[observation]`
- `Evidence[proof]`
- `Evidence[necessity]`
- `Evidence[possibility]`

## Stage 0 Promotion Law

Stage 0 does not permit implicit modal promotion. Every modal transition must be
an explicit builtin call with the required evidence token.

### Supported Promotions

1. `Unknown[T] -> Hypothesis[T]` requires `Evidence[observation]`
2. `Hypothesis[T] -> Fact[T]` requires `Evidence[proof]`
3. `Fact[T] -> Necessary[T]` requires `Evidence[necessity]`
4. `Unknown[T] -> Possible[T]` requires `Evidence[possibility]`
5. `Possible[T] -> Fact[T]` requires `Evidence[proof]`

### Supported Eliminations

1. `Necessary[T] -> Fact[T]` via weakening
2. `Fact[T] -> T` via explicit reveal

## Design Constraints

- no implicit coercions
- no stochastic epistemics
- no effect-free forged promotion
- evidence kinds are finite and canonical
- promotion remains auditable by call-site inspection

## Stage 0 Surface Strategy

The parser does not yet own dedicated modal syntax. Stage 0 uses explicit
builtins so modal law can be enforced immediately without waiting for later
surface-language expansion.

This keeps P1-006 compatible with existing lexer/parser/type-checker slices
while still establishing the kernel law required by later semantics, proof,
and self-reference tasks.
