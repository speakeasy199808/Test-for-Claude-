# Design — P0-016 Versioning Strategy

## Versioning Model

### Semantic Versioning (SemVer 2.0.0)
All crates in the Lyra workspace follow [Semantic Versioning 2.0.0](https://semver.org/):
- **MAJOR** — incompatible API changes, constitutional invariant changes
- **MINOR** — backward-compatible functionality additions
- **PATCH** — backward-compatible bug fixes

### Workspace-Unified Version
All crates share a single version via `[workspace.package].version`. Individual crate versions diverge only when a crate reaches independent stability (not expected during Phase 0).

### Pre-1.0 Convention
During pre-1.0 development (`0.x.y`):
- MINOR bumps may include breaking changes
- PATCH bumps are backward-compatible fixes
- The `0.1.0` version indicates initial development

## Git Tagging Convention

### Tag Format
```
v{MAJOR}.{MINOR}.{PATCH}
```
Examples: `v0.1.0`, `v0.2.0`, `v1.0.0`

### Tag Rules
1. Tags are annotated (`git tag -a`)
2. Tag message includes changelog excerpt for the release
3. Tags are signed when GPG keys are available
4. Tags are applied to the merge commit on `main`

## Commit Message Convention

### Format: Conventional Commits 1.0.0
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
| `docs` | Documentation only | No bump |
| `style` | Formatting, no code change | No bump |
| `refactor` | Code change, no feature/fix | No bump |
| `perf` | Performance improvement | PATCH bump |
| `test` | Adding/correcting tests | No bump |
| `build` | Build system or dependencies | No bump |
| `ci` | CI configuration | No bump |
| `chore` | Maintenance tasks | No bump |

### Scopes
Scopes correspond to crate names: `k0`, `k1`, `lyralang`, `shells`, `slices`, or subsystem modules like `k0/codec`, `k0/genesis`.

### Breaking Changes
- Append `!` after type/scope: `feat(k0)!: redesign digest API`
- Or include `BREAKING CHANGE:` footer
- Breaking changes trigger MAJOR bump (or MINOR during pre-1.0)

## Release Branch Strategy

### Branch Model
- `main` — always releasable, protected
- `blackboxai/**` — agent development branches
- `release/v{X}.{Y}` — stabilization branches (created when preparing a release)
- `feat/**` — feature branches (merged via PR to main)
- `fix/**` — bugfix branches

### Release Process
1. Create `release/v{X}.{Y}` from `main`
2. Only bug fixes and docs allowed on release branch
3. Update `CHANGELOG.md` with release notes
4. Bump version in `Cargo.toml` workspace
5. Tag the release commit
6. Merge release branch back to `main`

## Design Decisions
1. **Workspace-unified version** — simplifies coordination during early development
2. **Conventional Commits** — enables automated changelog generation
3. **Annotated tags** — provides audit trail for releases
4. **Main-based flow** — keeps `main` always releasable, release branches for stabilization only
