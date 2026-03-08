# Contributing to Lyra

## Commit Message Convention

Lyra uses [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description | Version Impact |
|---|---|---|
| `feat` | New feature | MINOR bump |
| `fix` | Bug fix | PATCH bump |
| `docs` | Documentation only | None |
| `style` | Formatting, no code change | None |
| `refactor` | Code restructuring, no feature/fix | None |
| `perf` | Performance improvement | PATCH bump |
| `test` | Adding or correcting tests | None |
| `build` | Build system or dependency changes | None |
| `ci` | CI configuration changes | None |
| `chore` | Maintenance tasks | None |

### Scopes

Use crate names as scopes: `k0`, `k1`, `lyralang`, `shells`, `slices`.
For subsystem specificity: `k0/codec`, `k0/genesis`, `k0/digest`, etc.

### Examples

```
feat(k0/entropy): add receipt chain verification

fix(k0/codec): correct varint decoding for max u64

docs: update VERSIONING.md with pre-1.0 rules

refactor(k0/time): extract clock merge into separate method

ci: add k0 unit test isolation job

feat(k0)!: redesign digest routing API

BREAKING CHANGE: DigestOutput now requires algorithm tag
```

## Development Workflow

### Branch Naming
- `feat/<description>` — new features
- `fix/<description>` — bug fixes
- `docs/<description>` — documentation
- `refactor/<description>` — refactoring
- `blackboxai/<description>` — AI agent branches

### Pull Request Process
1. Create a feature/fix branch from `main`
2. Make changes following project conventions
3. Ensure all CI gates pass locally:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. Write a clear PR description referencing the task ID (e.g., P0-018)
5. Request review (dual review required for k0 changes — see P0-020)
6. Squash-merge or rebase-merge to `main`

## Code Standards

### Mandatory for All Crates
- `#![forbid(unsafe_code)]`
- `#![deny(missing_docs)]`
- `#![deny(clippy::all)]`

### Testing
- Unit tests inline with modules (`#[cfg(test)]`)
- Integration tests in `tests/` directories
- Property-based tests with `proptest` where appropriate
- All tests must be deterministic — no wall clock, no unseeded randomness

### Documentation
- Every public item must have a doc comment
- Module-level documentation explaining purpose and ownership
- Task IDs referenced in module docs (e.g., `(P0-008)`)

## Constitutional Invariants

When contributing to `k0/`, these invariants must be preserved:

1. **Determinism** — identical inputs always produce identical outputs
2. **No ambient nondeterminism** — no `std::time`, no unseeded randomness
3. **Canonical serialization** — all encoding is deterministic and versioned
4. **Offline-first** — core truth requires no network access
5. **Explicit ownership** — code lives in the crate that owns the behavior
