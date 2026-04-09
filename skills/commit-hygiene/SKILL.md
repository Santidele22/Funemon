---
name: commit-higiene
description: Conventional Commits - mensajes de commit estructurados y significativos. Formato: type(scope): description.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: commit-conventions
---

## ¿Qué soy?

Soy el standard de convencional commits. Aseguro que cada commit sea significativo, rastreable, y automatizable.

## Formato de Commit

```
<type>(<scope>): <description>

[opcional body]

[opcional footer]
```

## Tipos de Commits

| Type | Descripción | Ejemplo |
|------|-------------|---------|
| **feat** | Nueva funcionalidad | `feat(auth): add github oauth` |
| **fix** | Bug fix | `fix(api): validate email format` |
| **docs** | Documentación | `docs(readme): update install` |
| **style** | Formateo sin lógica | `style(indent): fix spaces` |
| **refactor** | Código sin cambios funcionales | `refactor(auth): extract validate` |
| **test** | Agregar/modificar tests | `test(auth): add oauth tests` |
| **chore** | Mantenimiento | `chore(deps): update rust` |
| **perf** | Performance | `perf(api): cache user data` |
| **ci** | CI/CD | `ci(github): add test workflow` |
| **build** | Build system | `build(cargo): update version` |

## scopes Comunes

```
scope populares:
- auth
- api
- db
- ui
- cli
- config
- docs
- test
- deps
- ci
```

## Ejemplos

### Feature
```
feat(auth): add github oauth login

- Add OAuth2 flow with GitHub provider
- Store tokens in secure cookie
- Add user session management
```

### Bug Fix
```
fix(api): validate email before save

- Add email format validation
- Return 400 on invalid email
- Add test for invalid formats
```

### Refactor
```
refactor(auth): separate validate from business logic

- Extract validate_user() to module
- Improve error messages
- No functional changes
```

### Breaking Change
```
feat(api): change user response format

BREAKING CHANGE: /api/user now returns {data: user}
Instead of {user: user}

Migration guide in docs/migration.md
```

## Reglas de Commits

### ✅ REGlas

1. **Primera línea: máximo 50 caracteres**
2. **Tipo en minúsculas**
3. **Scope opcional pero recomendado**
4. **Descripción en imperativo** ("add" no "added")
5. **Body si hay más de 72 caracteres**
6. **Footer para breaking changes**

### ⚠️ PROHIBICIONES

1. **NO usar "fix stuff" o "updates"**
2. **NO commit sin mensaje**
3. **NO commit con TODO en mensaje**
4. **NO commbear múltiples features en uno**
5. **NO commit vacío ("wip")**

## Conventional Commits Automatizado

### Herramientas

```bash
# Instalación
npm install -g commitizen

# Uso interactivo
git cz

# Validar commit message
npx commitlint
```

### Git Hook

```bash
# .husky/commit-msg
npx -- commitlint --edit "$1"
```

## Commits por Feature

```
Regla: Un feature = Un commit o squash group

SI un feature requiere muchos commits:
- rebase antes de PR
- squash en merge

NO:
feat: add login
feat: add register  
feat: add logout

SÍ:
feat(auth): add oauth login
```

## Checklist Pre-Commit

- [ ] Mensaje sigue conventional commits
- [ ] Solo cambios relacionados
- [ ] No hay archivos sensibles
- [ ] Tests pasando
- [ ] [ ] tiene 50 caracteres o menos
- [ ] Descripción clara

## Gitmoji (Opcional)

```
✨ feat
🐛 fix
📝 docs
🎨 style
♻️ refactor
⚡️ perf
✅ test
🔧 chore
🚧 WIP
```

## Triggers

Este skill se activa cuando:
- Se menciona "commit"
- Se va a ejecutar `git commit`
- Se menciona "git", "commit message"

## Autonomía

Como siempre, pregunto antes de commit:

1. **Antes de git add** → Te pregunto qué archivos
2. **Antes de commit** → Te muestro el mensaje para approval
3. **Si mensaje no sigue formato** → Te sugiero formato correcto

## Mensajes Inválidos

```
❌ NO:
- "fix"
- "updates"
- "wip"
- "fixed stuff"
- "asdf"
- ""
- "TODO"
```

```
✅ SÍ:
- "feat(auth): add github oauth"
- "fix: resolve login redirect"
- "docs(readme): update install steps"
```