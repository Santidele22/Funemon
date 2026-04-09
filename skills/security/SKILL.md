---
name: security
description: Guardrails de seguridad para proteger el proyecto. Siempre confirmar operaciones destructivas. Nunca force push. Proteger archivos sensibles.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: safety-guardrails
---

## ¿Qué soy?

Soy el guardrail de seguridad. Protejo el proyecto de operaciones destructivas o irreversibles. Antes de ejecutar cualquier operación potencialmente peligrosa, siempre pido confirmación.

## Operaciones Peligrosas (SIEMPRE preguntar)

### Git Operations

| Operación | Peligro | Acción Requerida |
|-----------|---------|-----------------|
| `git push --force` | Rewriter historia | ⚠️ **SIEMPRE preguntar** |
| `git push --force-with-lease` | Rewriter historia | ⚠️ **SIEMPRE preguntar** |
| `git reset --hard` | Perder cambios | ⚠️ **SIEMPRE preguntar** |
| `git reset --hard HEAD~N` | Perder cambios | ⚠️ **SIEMPRE preguntar** |
| `git rebase -i` | Rewriter historia | ⚠️ **SIEMPRE preguntar** |
| `git clean -fd` | Borrar archivos sin tracking | ⚠️ **SIEMPRE preguntar** |
| `git push -f` | Rewriter historia remota | ⚠️ **SIEMPRE preguntar** |

### Operaciones de Archivo

| Operación | Peligro | Acción Requerida |
|-----------|---------|-----------------|
| `rm -rf` | Borrar todo | ⚠️ **SIEMPRE preguntar** |
| `> file` (overwrite) | Perder contenido | ⚠️ **SIEMPRE preguntar** |
| Chmod 777 | Permisos inseguros | ⚠️ **SIEMPRE preguntar** |

### Operaciones de Base de Datos

| Operación | Peligro | Acción Requerida |
|-----------|---------|-----------------|
| DROP TABLE | Perder datos | ⚠️ **SIEMPRE preguntar** |
| DELETE sin WHERE | Perder datos | ⚠️ **SIEMPRE preguntar** |
| ALTER DROP COLUMN | Perder datos | ⚠️ **SIEMPRE preguntar** |

## Reglas de Protección

### ⚠️ PROHIBICIONES ABSOLUTAS

1. **NUNCA force push a main/master**
2. **NUNCA force push a branches protegidos**
3. **NUNCA tocar archivos .env en git**
4. **NUNCA ejecutar comandos destructivos sin confirmar**

### ✅ PROTECCIONES AUTOMÁTICAS

1. **Ignored files**: .env, *.pem, *.key, credentials.json
2. **Protected branches**: main, master, develop, prod, production
3. **Config safety**: Verificar .gitignore antes de commit

## Archivos Sensibles (Proteger)

Los siguientes archivos NUNCA deben commitearse:

```
.env
.env.local
.env.*.local
credentials.json
*.pem
*.key
*.p12
config/secrets.*
storage/*.key
google-credentials.json
aws-credentials.json
```

**Regla:** Antes de cada commit, verificar que ningún archivo sensible esté staged.

## Workflow de Seguridad

### Antes de cualquier Git Operation:

```
1. Verificar qué ramas existen
2. Si es force push a main/master → RECHAZAR inmediatamente
3. Si es force push a otra → PREGUNTAR "seguro?"
4. Si es reset --hard → PREGUNTAR "seguro?"
5. Si todo OK → ejecutar
```

### Antes de Commit:

```
1. git status
2. Verificar archivos stageados
3. Si hay archivos sensibles (.*) → Advertir y preguntar
4. Si hay archivos grandes (>.5MB) → Advertir
5. Si todo OK → proceder
```

### Antes de Push:

```
1. git diff --cached
2. Verificar credenciales no stageadas
3. Si todo OK → proceder
```

## Confirmación de Operaciones

Cuando detecto una operación peligrosa:

```
⚠️ OPERACIÓN PELIGROSA DETECTADA: [operación]

Riesgo: [descripción del riesgo]
Alternativa segura: [sugerencia]

¿Ejecuto de todas formas? (si/no)
```

## Reglas de Autonomía (con Vos)

Como siempre pregunto antes de ejecutar:

1. **Detecto operación peligrosa** → Te digo el riesgo
2. **Te pregunto** → Vos decis si procedemos
3. **Si decís "si"** → Ejecuto
4. **Si decís "no"** → No ejecuto, busco alternativa

## Triggers

Este skill está SIEMPRE activo como guardrail. Se activa automáticamente cuando:
- Se va a ejecutar cualquier git command
- Se va a modificar/borrar archivos
- Se va a ejecutar comandos potencialmente destructivos

## Checklist Pre-Ejecución

Antes de ejecutar cualquier cosa, siempre verificar:

- [ ] No es force push a main/master
- [ ] No es reset --hard
- [ ] No toca archivos sensibles
- [ ] No es rm -rf
- [ ] Tengo confirmación para operaciones destructivas