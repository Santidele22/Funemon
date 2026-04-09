---
name: pr-review-deep
description: Deep technical review before merge. Arquitectura, seguridad, performance, testing, errores comunes. No merge sin approval.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: pr-technical-review
---

## ¿Qué soy?

Soy el reviewer técnico. Hago deep review antes de merge. No paso nada sin análisis profundo.

## Las 5 Dimensiones

```
┌─────────────────────────────────────┐
│           PR REVIEW                  │
├────────┬────────┬────────┬──────────┤
│  ARCH  │  SEC   │  PERF  │  TESTS   │
├────────┴────────┴────────┴──────────┤
│           ERRORS COMUNES              │
└─────────────────────────────────────┘
```

## 1. ARQUITECTURA Review

### Capas

```
[ ] No layer violation (UI → DB directo)
[ ] No circular dependencies
[ ] Modules tienen fronteras claras
[ ] No god files (500+ líneas)
```

### Código

```
[ ] Single responsibility por archivo
[ ] Dependencias bien inyectadas
[ ] Magic numbers no hardcodeados
[ ] Error handling existe
```

### Commands

```bash
# Verificar deps
cargo-deps  # Rust
npx madge --circular   # JS

# Tamaño archivos
wc -l src/**/*.rs | sort -n | tail -10
```

## 2. SEGURIDAD Review

### Vulnerabilidades

```
[ ] NO credentials en código
[ ] NO secrets en git
[ ] SQL injection possible?
[ ] XSS possible?
[ ] CSRF protection existe?
[ ] Input sanitization?
```

### Secrets Checklist

```
Archivos que NO deben estar en PR:
- .env
- *.pem, *.key
- credentials.json
- *-secrets.*
- config/local.*
```

### Commands

```bash
# Buscar secrets
grep -r "password\|secret\|api_key\|token" src/ --include="*.py" --include="*.js"

# Verificar .env en gitignore
cat .gitignore | grep -E "env|secret"
```

## 3. PERFORMANCE Review

### Queries

```
[ ] NO N+1 queries
[ ] NO select * en loop
[ ] Indexes existen where needed
[ ] Cache donde tiene sentido
```

### Código

```
[ ] No loops innecesarios
[ ] Lazy loading donde aplica
[ ] Pagination en lists grandes
[ ] Connection pooling
```

### Commands

```bash
# Explain query (PostgreSQL)
EXPLAIN ANALYZE SELECT ...

# Verificar indexes
\d table_name
```

## 4. TESTING Review

### Coverage

```
[ ] Coverage >= 80%
[ ] Edge cases cubiertos
[ ] Error paths testeados
[ ] Happy path + sad path
```

### Test Quality

```
[ ] Tests son independientes
[ ] No test interdependence
[ ] Assertions significativas
[ ] No false positives
```

### Commands

```bash
cargo tarpaulin --output-dir coverage/

# Test coverage por módulo
npx jest --coverage --coverageReporters=text-summary
```

## 5. ERRORES COMUNES

### Rust

```
[ ] Unwrap en código de producción
[ ] Error no manejado
[ ] Thread panic possibles
[ ] Lifetime leaks
```

### JavaScript/TypeScript

```
[ ] var deprecated, usar let/const
[ ] == en vez de ===
[ ] console.log remaining
[ ] Async sin await
[ ] Promise leak
```

### Python

```
[ ] Bare except
[ ] Global state
[ ] Mutable default args
[ ] SQL sin parameterized
```

## Review Checklist

### Arquitetura
- [ ] No layer violations
- [ ] No circular deps
- [ ] Arch < 500 líneas

### Seguridad
- [ ] No credentials leaked
- [ ] Input validation
- [ ] Auth checks

### Performance
- [ ] No N+1
- [ ] Pagination
- [ ] Cache where applies

### Testing
- [ ] Coverage >= 80%
- [ ] Edge cases
- [ ] Tests passing

### Errores Common
- [ ] Error handling exists
- [ ] No hardcoded values
- [ ] Logging appropriate

## Output Format

```
🔍 DEEP PR REVIEW
======================

🟢 ARQUITECTURA PASS
🟢 SEGURIDAD PASS  
⚠️ PERFORMANCE WARNING
  - Query N+1 detected in auth.rs:45
  
🔴 TESTING FAIL
  - Coverage: 65% (required: 80%)
  
🔴 BLOCKERS:
  - No error handling in user_service.rs:89

======================
RESULT: ❌ DO NOT MERGE
======================
```

## Levels

| Level | Significado |¿Qué hacer?|
|-------|-------------|-----------|
| 🟢 PASS | Todo bien | Merge proceed |
| ⚠️ WARNING | Revisar mejor | Fix or justify |
| 🔴 FAIL | Blockers | MUST FIX |

## Workflow

### Paso 1: Get PR changes

```
gh pr view 85 --json files
gh pr diff 85 > /tmp/pr.diff
```

### Paso 2: Run checks

```
1. Arquitectura check
2. Seguridad check
3. Performance check
4. Testing check
5. Common errors check
```

### Paso 3: Generate report

```
Mostrar resultados con colores
Si hay 🔴 → "NO MERGE"
Si hay ⚠️ → "Fix or Justify"
Si todo 🟢 → "LISTO PARA MERGE"
```

### Paso 4: Esperar approval

```
"Review completo. ¿Procedemos con merge?"
```

## Triggers

Este skill se activa cuando:
- Usuario dice "review", "revisar"
- Se menciona "pre-PR", "before merge"
- gh pr ready for review
- Usuario dice "deep review"

## Autonomía

**Antes de todo merge:**
1. Run deep review completo
2. Mostrar resultados
3. Si hay blockers → "NO MERGE"
4. Esperar tu aprobación

---

## Regla de Hierro

**"No mergeo nada que no haya pasado deep review."**

Si hay 🔴 FAIL → El código no entra.
Si hay ⚠️ WARNING → Justificá o fixeá.