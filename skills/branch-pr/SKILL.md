---
name: branch-pr
description: Git branch workflow + PR lifecycle. Crear branch, commits, PR. Protected branches, clean workflow.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: git-branch-pr
---

## ¿Qué soy?

Soy el workflow de branch + PR. Gestiono el ciclo de vida completo: crear branch → trabajar → commits → PR → merge.

## Protected Branches

**NUNCA trabajar directamente en estos branches:**

```
main
master
develop
production
prod
release/*
```

## Workflow de Branch

### Paso 1: Verificar Estado

```bash
# Ver estado actual
git status
git branch -a

# Ver cambios pendientes
git diff
git diff --cached
```

### Paso 2: Crear Branch

```
Branch naming:
- feature/[ticket]-description    → feature/JIRA-123-oauth-login
- bugfix/[ticket]-description    → bugfix/JIRA-456-fix-login
- hotfix/[ticket]-description  → hotfix/JIRA-789-security-patch
- refactor/[description]      → refactor-clean-auth
```

```bash
# Desde main/master
git checkout main
git pull origin main

# Crear branch
git checkout -b feature/ticket-descripcion
```

### Paso 3: Trabajar

```
Workflow:
1. git add .
2. git commit -m "feat: add oauth login"
3. git push -u origin feature/ticket-descripcion
```

### Paso 4: Pull Request

```bash
# Antes de PR, verificar:
git diff main...
git log main..feature/ticket

# Ejecutar tests
cargo test
```

### Paso 5: Crear PR

```bash
gh pr create --title "feat: description" --body "## Summary\n- description\n\n## Type\nFeature"
```

O usar:
```markdown
## Summary
[descripción corta]

## Changes
- [cambio 1]
- [cambio 2]

## Type
Feature/Bugfix/Refactor

## Tests
[tests que pasan]
```

### Paso 6: Merge

**NUNCA hacer merge sin tu aprobación.**

```
PR Merging Options:
- Squash and merge   → squash commits en uno
- Rebase and merge  → rebase sobre main
- Merge commit      → mantener historia

Recomendado: Squash and merge
```

## Rama Base

| Tipo de Branch | Rama Base |
|----------------|-----------|
| feature/* | main |
| bugfix/* | main |
| hotfix/* | main |
| release/* | main |

## Commit Pattern

```
<type>(<scope>): <description>

Types:
- feat:     nueva funcionalidad
- fix:      bug fix
- docs:     documentación
- style:    formatting
- refactor: restructure código
- test:     agregar tests
- chore:    mantenimiento

Ejemplos:
feat(auth): add github oauth login
fix(api): validate email before save
docs(readme): update installation
```

## Rebase vs Merge

| Operación | Cuándo | Cómo |
|-----------|--------|------|
| **Rebase** | Before push, mantener historia limpia | `git rebase main` |
| **Merge** | Cuando el feature está listo | `git merge feature/main` |

### Rebase Workflow

```bash
# 1. Guardar cambios actuales
git stash

# 2. Traer cambios de main
git fetch origin
git rebase origin/main

# 3. Resolver conflictos
# [editar archivos en conflicto]
git add .
git rebase --continue

# 4. Aplicar cambios
git stash pop
```

## Clean Up

```bash
# Eliminar branch local mergeada
git branch -d feature/xxx

# Eliminar branch remote mergeada
git push origin --delete feature/xxx
```

## Checklist Pre-PR

- [ ] Tests pasan
- [ ] No hay merge conflicts
- [ ]Branch naming correcto
- [ ] Commits siguiendo conventional commits
- [ ]Coverage >= 80%
- [ ] Documentación actualizada
- [ ]Te pregunté antes de crear PR

## Triggers

Este skill se activa cuando:
- Se menciona "branch", "pr", "merge", "push"
- Se va a crear nuevo branch
- Se va a hacer push
- Se va a crear Pull Request

## Reglas de Autonomía

Como siempre, pregunto antes de ejecutar:

1. **Antes de crear branch** → Te muestro el nombre y la base
2. **Antes de commit** → Te muestro el mensaje
3. **Antes de push** → Te muestro qué se push
4. **Antes de PR** → Te muestro el PR completo
5. **Antes de merge** → Te pregunto "procedemos?"

## Git Safety

Con skill `security` activo:
- ⚠️ **NUNCA force push a protected branches**
- ⚠️ **NUNCA hacer reset --hard sin confirmar**
- ✅ Siempre verificar antes de cualquier operación