# LyraLang Grammar — Stage 0 Foundations

## Status

This document is the normative Stage 0 language grammar surface for Phase 1 work.
P1-001 defines lexical structure here first; P1-002 extends this same document with the seed syntactic grammar consumed by P1-016.

## 1. Source Normalization

Lyra source text is normalized before tokenization.

1. `CRLF` (`\r\n`) is normalized to `LF` (`\n`).
2. Lone `CR` (`\r`) is normalized to `LF` (`\n`).
3. Token spans, line numbers, and columns are defined over the normalized source.
4. Horizontal whitespace is preserved as trivia; it is never silently deleted inside string literals.
5. Newlines are explicit lexical separators after normalization.

This normalization rule is deterministic and platform-independent.

## 2. Identifiers

Identifiers are Unicode-aware.

### 2.1 Identifier Start

An identifier start code point is one of:

- `_`
- any Unicode code point satisfying `XID_Start`

### 2.2 Identifier Continue

An identifier continuation code point is one of:

- `_`
- any Unicode code point satisfying `XID_Continue`
- ASCII digits `0-9`

### 2.3 Identifier Notes

- Identifiers are case-sensitive.
- Unicode normalization is not rewritten by the lexer; source is compared exactly as written after line-ending normalization.
- A single `_` is tokenized as the wildcard underscore token.
- Reserved words are not accepted as ordinary identifiers.

### 2.4 Examples

Valid identifiers:

- `value`
- `_`
- `_shadow`
- `café`
- `Δelta`
- `变量`

## 3. Reserved Words

The following words are reserved in Stage 0:

- `as`
- `break`
- `continue`
- `effect`
- `else`
- `false`
- `fn`
- `for`
- `if`
- `impl`
- `import`
- `in`
- `let`
- `loop`
- `match`
- `module`
- `proof`
- `return`
- `trait`
- `true`
- `type`
- `use`
- `where`
- `while`
- `with`

Reserved words always tokenize as keyword tokens, never as identifiers.

## 4. Comments

Lyra supports two comment forms.

### 4.1 Line Comments

A line comment begins with `//` and continues until but does not include the next normalized newline.

Example:

```text
// this is a comment
```

### 4.2 Block Comments

A block comment begins with `/*` and ends with `*/`.

- Block comments may span multiple lines.
- Nested block comments are permitted.
- Unterminated block comments are lexing errors.

Example:

```text
/* outer /* inner */ outer */
```

## 5. Whitespace

Whitespace is split into two categories:

- **horizontal whitespace** — spaces, tabs, and other non-newline Unicode separators; emitted as trivia
- **newline** — normalized `LF`; emitted as `Newline`

Whitespace outside string literals has no semantic payload at the lexical layer beyond token separation and span tracking.

## 6. Literals and Delimiters in the Seed Lexer

The seed lexer also recognizes the following foundational token families so later parser tasks can build on a stable token stream:

- integer literals: ASCII decimal digits with optional `_` separators
- string literals: double-quoted UTF-8 text with backslash escapes
- delimiters: `(` `)` `{` `}` `[` `]`
- punctuation/operators: `,` `:` `;` `.` `@` `?` `+` `-` `*` `/` `%` `=` `==` `!=` `<` `<=` `>` `>=` `->` `=>` `::` `&` `&&` `|` `||`

These token families are intentionally limited to the seed compiler surface and will be refined by later syntax tasks.

## 7. Syntactic Overview

Stage 0 Lyra is expression-oriented.

- blocks are expressions
- `if` is an expression
- `match` is an expression
- `let` introduces bindings as statements in program or block position
- programs and blocks may end in a trailing expression without a terminator

This surface is intentionally minimal: it is sufficient for parser bring-up and stable AST construction, not yet the full language.

## 8. Program and Block Structure

A source file is a program.

- a program may start with an optional `module` declaration
- statements are separated by normalized newlines or `;`
- the final form in a program or block may be an unterminated expression, which becomes the tail expression of that container
- `let` bindings are statements; expressions may appear either as terminated statements or as the tail expression

## 9. EBNF Grammar

The following grammar is normative for the Stage 0 parser.

```ebnf
program          = separators , [ module_decl , separators ] , form_seq , EOF ;
module_decl      = "module" , identifier ;
form_seq         = { form , separators } , [ expression ] ;
form             = let_stmt | expr_stmt ;
let_stmt         = "let" , pattern , "=" , expression ;
expr_stmt        = expression ;
separators       = { newline | ";" } ;

expression       = logic_or ;
logic_or         = logic_and , { "||" , logic_and } ;
logic_and        = equality , { "&&" , equality } ;
equality         = comparison , { ("==" | "!=") , comparison } ;
comparison       = additive , { ("<" | "<=" | ">" | ">=") , additive } ;
additive         = multiplicative , { ("+" | "-") , multiplicative } ;
multiplicative   = prefix , { ("*" | "/" | "%") , prefix } ;
prefix           = [ "-" ] , postfix ;
postfix          = primary , { call_suffix } ;
call_suffix      = "(" , [ argument_list ] , ")" ;
argument_list    = expression , { "," , expression } ;
self_reference   = "@" , ("current_program" | "current_receipt" | "ledger_state") , "(" , ")" ;

primary          = identifier
                 | integer
                 | string
                 | boolean
                 | self_reference
                 | grouped
                 | block
                 | if_expr
                 | match_expr ;

grouped          = "(" , expression , ")" ;
block            = "{" , separators , form_seq , separators , "}" ;
if_expr          = "if" , expression , expression , [ "else" , expression ] ;
match_expr       = "match" , expression , "{" , separators , match_arm_list , separators , "}" ;
match_arm_list   = match_arm , { separators , [ "," ] , separators , match_arm } , [ separators , "," ] ;
match_arm        = pattern , "=>" , expression ;

pattern          = wildcard_pattern
                 | identifier_pattern
                 | integer_pattern
                 | string_pattern
                 | boolean_pattern ;

wildcard_pattern   = "_" ;
identifier_pattern = identifier ;
integer_pattern    = integer ;
string_pattern     = string ;
boolean_pattern    = "true" | "false" ;
```

## 10. Operator Precedence and Associativity

From lowest binding power to highest:

1. `||`
2. `&&`
3. `==`, `!=`
4. `<`, `<=`, `>`, `>=`
5. `+`, `-`
6. `*`, `/`, `%`
7. prefix `-`
8. call suffix `(...)`

All infix operators in this seed grammar are left-associative.

## 11. Pattern Matching Constraints

The Stage 0 parser supports a deliberately small pattern language.

- wildcard pattern: `_`
- identifier pattern: `name`
- literal patterns: integer, string, `true`, `false`

Exhaustiveness, guards, or destructuring are deferred to later tasks.

## 12. AST Requirements

The seed parser shall emit a span-carrying AST.

- every program node has a source span
- every statement has a source span
- every expression has a source span
- every pattern and match arm has a source span
- spans are defined over normalized source coordinates emitted by the lexer

## 13. Determinism Requirements

The parser must:

- consume tokens in deterministic source order
- preserve stable operator precedence and associativity
- produce the same AST shape for identical token streams
- emit diagnostics in deterministic order
- keep span construction canonical and derived only from normalized source coordinates

## 14. Deferred Grammar Surface

Later Phase 1 tasks extend this document with:

- concrete type and effect surface syntax (P1-003, P1-004)
- semantic and typing judgments (P1-008 onward)
- richer patterns and exhaustiveness checking (P1-012)
- syntax extension and metaprogramming (P1-026, P1-027)


## 15. Type Kernel Linkage

The concrete source grammar deliberately stops short of user-written type syntax in Stage 0.
The normative internal type algebra consumed by the seed checker is defined in `docs/lyralang/TYPES.md`.
That document fixes the canonical representations for primitive, product, sum, and function types,
along with Hindley-Milner schemes and deterministic variable allocation rules.


## Self Reference Extension

Stage 0 also reserves the zero-argument self-reference primitives described in `docs/lyralang/SELF_REFERENCE.md`. Their `@name()` surface is parsed directly and lowered to dedicated codegen IR instructions.


## Error-Handling Extension

Stage 0 admits postfix propagation with `?` on the existing expression surface.

- postfix `?` binds at the same suffix layer as calls
- `call()?` and `value?` are valid Stage 0 forms
- propagation semantics are defined in `docs/lyralang/ERRORS.md`

## Stage 0 Conservative Concurrency and Temporal Surfaces

Until later grammar-expansion tasks land richer syntax, the executable Stage 0 parser admits concurrency and temporal logic through ordinary call expressions:

- `spawn(expr)`, `join(task)`, `select(task_a, task_b)`
- `channel_int()`, `send_int(channel, value)`, `recv_int(channel)`
- `always(expr)`, `eventually(expr)`, `until(lhs, rhs)`, `since(lhs, rhs)`
