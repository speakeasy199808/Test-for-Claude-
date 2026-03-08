# LyraLang Type-Level Computation (P1-028)

## Overview

The LyraLang type-level computation system provides const generics, type
families, and compile-time arithmetic with termination checking. In Stage 0
the system is recognized conservatively through builtin call forms and
let-binding patterns over the existing parser surface.

## Surface Forms

### Const Generic Bindings

Let bindings whose right-hand side is an integer or boolean literal are
recognized as const generic parameters.

```lyra
let n = 10   -- ConstGeneric { name="n", value_type="Int", value_summary="10" }
let flag = true  -- ConstGeneric { name="flag", value_type="Bool", value_summary="true" }
```

### `const_add(a, b)`

Compile-time addition. When both arguments are literal constants, the
result is computed and `termination_reason = "constant_folding"`. When
arguments contain free variables, a `NonTerminatingComputation` error is
produced.

```lyra
let sum = const_add(10, 20)  -- result = "30", terminates = true
```

### `const_mul(a, b)`

Compile-time multiplication. Same termination rules as `const_add`.

```lyra
let product = const_mul(3, 7)  -- result = "21", terminates = true
```

### `type_family_define(name, params, result)`

Defines a type family with the given parameter kinds and result kind. The
definition is registered in scope for subsequent `type_family_apply` calls.

```lyra
let tf = type_family_define("Vec", "Nat", "Type")
```

### `type_family_apply(name, args...)`

Applies a registered type family. The first argument is the family name;
remaining arguments are the type arguments. Applying an undefined family
produces an `UndefinedTypeFamily` error.

```lyra
let applied = type_family_apply("Vec", 10)  -- result = "Vec[10]"
```

## Termination Checking

| Condition | `terminates` | `termination_reason` |
|-----------|-------------|----------------------|
| All arguments are literals | `true` | `"constant_folding"` |
| Nested const arithmetic over literals | `true` | `"bounded_recursion"` |
| Argument contains a free variable | `false` | `"unbounded_recursion"` |
| `type_family_apply` over known family | `true` | `"structural"` |
| `type_family_apply` over unknown family | `false` | `"undefined_family"` |

`all_terminate = true` iff no `NonTerminatingComputation` errors occurred.

## Error Kinds

| Kind | Condition |
|------|-----------|
| `ParseError` | Source could not be parsed. |
| `NonTerminatingComputation` | A const computation cannot be proved to terminate. |
| `KindMismatch` | A type family was applied with the wrong parameter kinds. |
| `UndefinedTypeFamily` | `type_family_apply` refers to a name not defined by `type_family_define`. |

## Checker API

```rust
use lyralang::typelevel::check;

let output = check(source);
// output.judgment.all_terminate -- true iff every computation terminates
// output.errors                  -- Vec<TypeLevelError>
```

## Stage 0 Scope

Stage 0 recognizes the surface forms and performs:
- Const generic binding classification.
- Compile-time arithmetic with literal-argument constant folding.
- Type family registration and application.
- Conservative termination checking.
- Error recovery on non-fatal errors.

Higher-kinded type families, dependent types, and full termination proof
search are planned for later stages.
