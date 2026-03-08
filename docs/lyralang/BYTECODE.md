# LyraLang Seed Bytecode Emitter — Stage 0

## Purpose

P1-020 defines the first canonical bytecode object emitted from the Stage 0 IR
introduced by P1-019. The emitter does not invent a second lowering pipeline; it
serializes the deterministic IR into a version-stamped LyraVM payload using the
canonical LyraCodec encoder from `k0` (P0-007).

## Canonical Format Tag

```text
lyravm-bytecode-v1
```

## Encoding Law

The bytecode object is serialized as a LyraCodec `Struct` with schema version `1`.
Field identifiers are fixed and must not be renumbered. Canonical ordering is
enforced by the encoder itself.

### Top-Level Fields

1. `format_version : String`
2. `module : String`
3. `ir_format_version : String`
4. `result_type : String`
5. `register_count : UInt`
6. `entry_register : UInt`
7. `instruction_count : UInt`
8. `instructions : Vector<Struct>`

### Instruction Fields

Each instruction is serialized as schema version `1` with:

1. `opcode : String`
2. `operands : Vector<String>`
3. `text : String`

The textual rendering is retained so bytecode inspection remains stable and
auditable even before a richer opcode registry exists.

## Instruction Normalization

The emitter consumes the canonical text form produced by P1-019 and derives:

- a stable opcode string
- a stable operand list
- the original canonical text

No reparsing of source syntax occurs at this stage. The IR text is already the
authoritative Stage 0 lowering boundary.

## Determinism Requirements

The seed emitter must:

- preserve source-ordered instruction sequence
- preserve the IR format tag emitted by P1-019
- encode identical IR into identical bytes
- reject malformed canonical IR with stable diagnostics
- produce a version-stamped object boundary suitable for later VM loading

## Boundary of This Task

This task defines the first canonical bytecode object only. It does **not** yet
execute bytecode, verify control-flow graphs, or perform optimization passes.
Those belong to later VM/runtime tasks.
