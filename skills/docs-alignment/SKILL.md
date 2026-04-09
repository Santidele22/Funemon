---
name: docs-alignment
description: Docs must match code. Verificar que documentación refleja el código real. Alertar cuando diverge.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: documentation-consistency
---

## ¿Qué soy?

Soy el guardián de docs-alignment. Me aseguro de que la documentación refleje el código real. Si diverge, alert.

## Regla de Hierro

**"Documentation is a contract with the future."**

Si el doc dice X pero el código hace Y → El doc está mal, no el código.

## Verificaciones

### 1. Antes de Commit/PUsh

```
CHECKLIST:
- [ ] Nuevo método exportado → hay doc?
- [ ] Parámetro nuevo → doc menciona?
- [ ] Return type cambió → doc actualizado?
- [ ] Example en doc → funciona hoy?
```

### 2. Tipos de Verificación

| Tipo | Qué | Cómo |
|------|-----|------|
| **Method signature** | Params, returns | Compare doc vs code |
| **Example code** | Ejecutar | `python -c "exec(doc example)"` |
| **API endpoint** | Request/response | Probar con curl/httpie |
| **Config example** | Parsear | Verificar válido |

## Errores Comunes

| Problema | Por qué falla |
|----------|---------------|
| Parámetro nuovo no documentado | Se agregó y no se actualizó doc |
| Return type cambió | API changed but readme no |
| Example outdated | Copy-paste de 版本 anterior |
| Deprecation no mentioned | Old way still in docs |

## Workflow

### Paso 1: Detectar cambios

```
git diff --name-only
→ filtrar archivos de docs:
  README.md, docs/*.md, API.md, CHANGELOG.md
```

### Paso 2: Para cada doc modificado

```
1. Extraer todos los code examples
2. Ejecutar cada uno
3. Si falla → ❌ blocker
4. Si no executa → alerta
```

### Paso 3: Para cada code modificado

```
1. Verificar tiene docstrings
2. Verificar pública API documentada
3. Si nuevo público → verificar doc existe
```

## Verificar Examples

```python
# Python - Extract y ejecutar
import re

def verify_doc_examples(doc_file):
    code_blocks = re.findall(r'```python\n(.*?)```', doc_file)
    for i, code in enumerate(code_blocks):
        try:
            exec(code, {})
        except Exception as e:
            print(f"Example {i} falla: {e}")
```

```bash
# Bash/CLI - Verificar comando
grep -A5 '```bash' docs/*.md | while read line; do
    echo "$line" | bash -n && echo "OK" || echo "FAIL: $line"
done
```

## Checklist Pre-PR

- [ ] Todos los examples ejecutan sin error
- [ ] Parámetros nuevos documentados
- [ ] Return types actualizados
- [ ] Deprecations mencionadas
- [ ] CHANGELOG.md actualizado si hay breaking

## Warning Types

| Level | Significado | Acción |
|-------|-----------|--------|
| 🔴 ERROR | Code no match doc | NO merge |
| 🟡 WARNING | Possible outdated | Revisar |
| 🔵 INFO | Suggestion | Considerar |

## Output Example

```
⚠️ DOCS ALIGNMENT ISSUES:

🔴 Method user_create() documented but doesn't exist in code
🟡 API.md example uses deprecated /v1/auth
🔵 CHANGELOG.md missing entry for PR #123
```

## Triggers

Este skill se activa cuando:
- Se menciona "docs", "documentation", "readme"
- Se va a hacer commit/push/PR
- Se modifica algún archivo .md en docs/
- Usuario dice "check doc", "verify doc"

## Autonomía

**Antes de todo commit:**
- Run docs-alignment check
- Mostrar resultados
- Esperar tu aprobación para proceder

---

## Ejemplo Real

```
git diff → modified:
- src/auth.rs
- docs/api.md

→ docs-alignment:
1. auth.rs: new public method `refresh_token()`
2. docs/api.md: NO mention of refresh_token

⚠️ WARNING: Public method added but not documented
- docs/api.md missing: refresh_token()

✅ Other examples pass
```

## Regla

**"If you can't document it, you can't ship it."**