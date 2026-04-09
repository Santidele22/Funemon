---
name: rust-release
description: Rust release workflow. Semantic versioning, Cargo.toml updates, changelog, publishing. Guide para releases.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: rust-release
---

## ¿Qué soy?

Soy el guide de releases en Rust. Te ayudo a hacer releases properos y semantic versioning.

## Versioning

### Semantic Versioning

```
MAJOR.MINOR.PATCH
  │     │     │
  │     │     └── Bug fixes
  │     └── new features (backward compatible)
  └── incompatible changes
```

### Version in Cargo.toml

```toml
[package]
version = "0.1.0"
```

### Bump Commands

```bash
# Patch bump (0.1.0 -> 0.1.1)
cargo version patch

# Minor bump (0.1.0 -> 0.2.0)
cargo version minor

# Major bump (0.1.0 -> 1.0.0)
cargo version major

# Set specific version
cargo version 1.2.3
```

## Release Workflow

### 1. Pre-Release Checklist

```bash
# Run tests
cargo test --all-features

# Check formatting
cargo fmt --check

# Lint
cargo clippy --all-targets

# Build release
cargo build --release
```

### 2. Update Version

```bash
cargo version minor
```

### 3. Update Changelog

```markdown
# Changelog

## [0.2.0] - 2024-01-15

### Added
- New feature X
- Command Y

### Changed
- Improved Z

### Fixed
- Bug in W
```

### 4. Git Tag

```bash
git add -A
git commit -m "release: v0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main
git push origin v0.2.0
```

### 5. Publish

```bash
# To crates.io
cargo publish

# To GitHub
git push --tags
```

## Cargo.toml Config

### package metadata

```toml
[package]
name = "mimir"
version = "0.1.0"
edition = "2021"
description = "Sistema de memoria persistente"
license = "MIT"
authors = ["Tu Nombre <tu@email.com>"]
repository = "https://github.com/user/repo"
readme = "README.md"

[package.metadata.release]
pre-release = false
```

### Workspace

```toml
[workspace]
members = ["crate1", "crate2"]

[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
```

## Changelog Automation

### git-chglog

```bash
# Install
cargo install git-chglog

# Configurar
git-chglog -c init

# Generar
git-chglog -o CHANGELOG.md
```

### conventional-changelog

```bash
# Generate changelog
conventional-changelog -p angular -i CHANGELOG.md -s

# Full release changelog
conventional-changelog -p angular -o CHANGELOG.md --from-v 0.1.0
```

## Best Practices

### ✅ SIEMPRE

- semantic versioning
- Tests pasan antes de release
- Changelog actualizado
- Git tags para releases
- version en Cargo.toml

### ✅ NUNCA

- No skip tests
- No release con warnings
- No forget changelog
- No force push a tags

### Release Checklist

- [ ] Tests pasan
- [ ] No Clippy warnings
- [ ] Documentation updated
- [ ] Version bumped
- [ ] Changelog updated
- [ ] Git tag created
- [ ] Published to registry

## Cargo.lock

### En Version Control?

**SÍ** -siempre en commits:
- Reproducible builds
- Exact same deps everywhere
- Security audit trail

```bash
git add Cargo.lock
git commit -m "chore: lock file update"
```

## Automation

### GitHub Actions

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/cargo@v1
        with:
          command: publish
      - uses: softprops/action-gh-release@v1
        with:
          generate_release_notes: true
```

### Release PR Template

```markdown
## Release v0.2.0

### Changes
- Feature X
- Bug fix Y

### Breaking Changes
None

### Dependencies
- Updated: `tokio` 1.x -> 2.x
```

## Triggers

Este skill se activa cuando:
- Making a release
- Updating version
- Creating changelog
- Publishing to crates.io

---

## Regla de Hierro

**"Tag temprano, tag a menudo. Siempre con changelog."**