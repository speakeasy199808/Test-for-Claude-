# Lyra Versioning Policy

## Semantic Versioning

All crates in the Lyra workspace follow [Semantic Versioning 2.0.0](https://semver.org/).

| Component | Bump When |
|---|---|
| **MAJOR** | Incompatible API changes, constitutional invariant changes, breaking protocol changes |
| **MINOR** | Backward-compatible new functionality, new modules, new public APIs |
| **PATCH** | Backward-compatible bug fixes, documentation corrections, performance improvements |

## Pre-1.0 Convention

During initial development (`0.x.y`):
- **MINOR** bumps (`0.x.0`) may include breaking changes
- **PATCH** bumps (`0.x.y`) are backward-compatible fixes only
- The system is not considered stable until `1.0.0`

## Workspace Version

All crates share a unified version defined in the root `Cargo.toml`:

```toml
[workspace.package]
version = "0.1.0"
```

Individual crates inherit via `version.workspace = true`. Version divergence is permitted only when a crate reaches independent stability.

## Git Tagging

### Format
```
v{MAJOR}.{MINOR}.{PATCH}
```

### Rules
1. Tags are **annotated**: `git tag -a v0.2.0 -m "Release v0.2.0"`
2. Tag message includes the changelog excerpt for the release
3. Tags are **signed** when GPG keys are configured
4. Tags are applied to the **merge commit on `main`**
5. Tags must not be moved or deleted after publication

## Version Bump Rules

### Constitutional Changes (k0)
Any change to k0's public API, constitutional invariants, or determinism guarantees:
- Pre-1.0: MINOR bump
- Post-1.0: MAJOR bump

### New Modules or Features
Adding new public modules, types, or functions:
- MINOR bump

### Bug Fixes and Internal Changes
Fixes that do not change public API surface:
- PATCH bump

### Documentation-Only Changes
No version bump required.

## Release Branch Strategy

| Branch | Purpose | Protection |
|---|---|---|
| `main` | Always releasable, primary integration target | Protected: requires PR + CI pass |
| `release/v{X}.{Y}` | Stabilization before tagging a release | Only bug fixes and docs |
| `blackboxai/**` | Agent development branches | CI runs on push |
| `feat/**` | Feature development | Merged via PR to `main` |
| `fix/**` | Bug fix development | Merged via PR to `main` |

### Release Process
1. Create `release/v{X}.{Y}` branch from `main`
2. Apply only bug fixes and documentation updates
3. Update `CHANGELOG.md` with release notes
4. Bump `[workspace.package].version` in root `Cargo.toml`
5. Run full CI pipeline and verify all gates pass
6. Tag the release commit: `git tag -a v{X}.{Y}.{Z}`
7. Merge release branch back to `main`
8. Push tag: `git push origin v{X}.{Y}.{Z}`
