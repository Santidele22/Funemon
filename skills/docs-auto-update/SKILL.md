---
name: docs-auto-update
description: Auto-actualiza documentación basado en cambios de código. Actualiza README cuando hay nuevos features, fixes o cambios.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: docs-auto-sync
---

## ¿Qué soy?

Soy el skill de documentación automática. Actualizo el README.md cuando hay cambios en el código.

## Funcionamiento

### Cuando actualizar

Se activa cuando:
- Nuevo commit en main/dev
- PR mergeada
- Nueva versión/tag
- Manual: "update docs"

### Qué actualizar

| Tipo de cambio | Sección a actualizar |
|---------------|-------------------|
| Nueva feature | Features |
| Bug fix | Bug fixes |
| Breaking change | Breaking changes |
| Nueva dependencia | Dependencias |
| Nueva command | Usage |
| Refactor | Migración |

## Auto-Detection

### Desde Commits

```bash
# Últimos commits desde último tag
git log --oneline $(latest_tag)..HEAD

# Analizar mensajes
feat: add oauth login  
fix: resolve token bug
docs: update readme
```

### Desde PRs

```bash
# Último PR mergeado
gh pr view --state merged --sort created -l merged --json title,body,additions,deletions
```

## Plantilla README

### Sections Dinámicas

```markdown
## 📦 Latest Changes

<!-- AUTO_UPDATE_START -->
### Nuevas Features
- OAuth login (PR #123)
- TUI interactive (PR #125)

### Bug Fixes
- Session fix (PR #124)

### Breaking Changes
- None
<!-- AUTO_UPDATE_END -->
```

## GitHub Actions

### Workflow

```yaml
# .github/workflows/update-docs.yml
name: Update Docs

on:
  push:
    branches: [main, dev]
  release:
    types: [published]

jobs:
  update-readme:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Generate changes
        run: |
          git log --oneline ${{ github.event.repository.default_branch }}..HEAD > changes.txt
          
      - name: Update README
        run: python scripts/update_readme.py changes.txt
          
      - name: Commit and PR
        run: |
          git add README.md
          git commit -m "docs: update README with latest changes"
          git push
```

### Script de Update

```python
#!/usr/bin/env python3
"""Update README.md con últimos cambios"""

import re
import sys
from pathlib import Path

def parse_commits(commits_file):
    """Parse commits desde archivo o git"""
    features = []
    fixes = []
    docs = []
    
    for line in commits_file.read().splitlines():
        if line.startswith('feat'):
            features.append(parse_feat(line))
        elif line.startswith('fix'):
            fixes.append(parse_fix(line))
        elif 'docs' in line:
            pass  # docs update docs, no update needed
            
    return features, fixes

def parse_feat(line):
    """Parse feature message"""
    # "feat: add oauth login (#123)"
    parts = line.split(': ')
    if len(parts) > 1:
        msg = parts[1].split(' (')[0]
        return f"- {msg}"
    return ""

def parse_fix(line):
    """Parse fix message"""
    parts = line.split(': ')
    if len(parts) > 1:
        msg = parts[1].split(' (')[0]
        return f"- {msg}"
    return ""

def update_readme(readme_path, features, fixes):
    """Actualiza README"""
    content = readme_path.read_text()
    
    # Find auto-update section
    start = content.find('<!-- AUTO_UPDATE_START -->')
    end = content.find('<!-- AUTO_UPDATE_END -->')
    
    if start == -1 or end == -1:
        print("No auto-update section found")
        return
        
    # Build new section
    new_section = f"""<!-- AUTO_UPDATE_START -->
### Nuevas Features
{chr(10).join(features) if features else '- Ninguna'}

### Bug Fixes
{chr(10).join(fixes) if fixes else '- Ninguno'}
<!-- AUTO_UPDATE_END -->"""
    
    # Replace
    content = content[:start] + new_section + content[end + len('<!-- AUTO_UPDATE_END -->'):]
    
    readme_path.write_text(content)

if __name__ == "__main__":
    changes = Path("changes.txt")
    if changes.exists():
        features, fixes = parse_commits(changes)
        update_readme(Path("README.md"), features, fixes)
```

## Auto-Detection de Commands

### Parseo de Cargo.toml

```python
def get_new_commands(cargo_diff):
    """Detectar nuevos commands"""
    new_commands = []
    for line in cargo_diff.splitlines():
        if line.startswith('+'):
            if '=' in line:
                cmd = line.split('=')[0].strip().strip('"')
                new_commands.append(cmd)
    return new_commands
```

## Integración con Mimir CLI

### Auto-detectar commands

```rust
// En Mimir, parsear commands:
// mimir --help -> list commands
// Comparar con README usage section
```

## Best Practices

### ✅ SIEMPRE

- Usar commit messages conventional
- Secciones delimitadas con HTML comments
- No overwrite manual content
- Backup antes de actualizar

### ✅ Auto-update Sections

```markdown
## 📦 Features

<!-- AUTO_FEATURES_START -->
<!-- AUTO_FEATURES_END -->

## 🐛 Bug Fixes

<!-- AUTO_FIXES_START -->
<!-- AUTO_FIXES_END -->
```

### ✅ NUNCA

- No cambiar secciones manuales
- No actualizar desde outside de sections
- No overwrite todo el README

## Trigger

Este skill se activa cuando:
- Nueva commit/push a main/dev
- Release publicada
- Usuario dice "update docs" o "sync readme"

---

## Regla de Hierro

**"Documentation reflects reality. Si el código cambió, el README debe cambiar."**