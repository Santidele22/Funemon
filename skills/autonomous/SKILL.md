---
name: autonomous
description: Workflow completo de autonomía: iniciar memoria → cargar contexto → SDD workflow → guardar decisiones → reflejar. Sin pedir permiso.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: autonomous-development
---

## ¿Qué soy?

Soy el workflow de autonomía completa. Gestiono el ciclo de vida entero de una sesión de desarrollo:
1. Iniciar memoria (sin preguntar)
2. Cargar contexto (sin preguntar)
3. Detectar tipo de trabajo (SDD / quick fix / debug)
4. Ejecutar según workflow
5. Guardar decisiones automáticamente
6. Reflexionar al cerrar

## Workflow Completo

### Paso 1: INIT (siempre primero)

**NUNCA pedir permiso. Ejecutar automáticamente.**

```bash
# 1. Detectar proyecto automáticamente
# Buscar en orden: package.json, Cargo.toml, pyproject.toml, .git/config

# 2. Iniciar sesión
memory_session_start(project: "nombre-del-proyecto")

# 3. Cargar contexto
memory_context(session_id: "ID-RECIBIDO", limit: 5)
```

### Paso 2: ANALYZE WORKFLOW

Analizar qué tipo de trabajo es:

| Si el usuario... | Ejecutar skill... |
|-----------------|-----------------|
| Pide spec / SDD / "definir" | `sdd` skill |
| Pide "debug" / "error" / "fix" | debug workflow |
| Pide "quick" / "simple" / "small" | quick fix workflow |
| Pide "refactor" / "clean" | refactor workflow |
| Pide "test" | TDD workflow |

### Paso 3: EXECUTE

Ejecutar el workflow correspondiente. Durante ejecución:

**Auto-guardar memorias:**
- Decisión de arquitectura → type: "plan"
- Error encontrado → type: "error"  
- Descubrimiento relevante → type: "observation"
- Preferencia del usuario → type: "preference"

```bash
# Guardar automáticamente cuando:
# - Resuelves un bug
# - Tomas decisión de diseño
# - Descubres algo relevante
# - El usuario expresa preferencia

memory_store(
  session_id: "ID",
  title: "Breve descripción",
  type: "error|plan|observation|preference",
  what: "Qué ocurrió",
  why: "Por qué es importante",
  learned: "Qué aprendiste"
)
```

### Paso 4: REFLECT (al cerrar)

**Cuando el usuario dice "eso es todo", " gracias", "cerrá", "fin":**

```bash
# 1. Generar reflexión externamente (analizar memorias)
# 2. Estructurar como JSON:
{
  "content": "Resumen consolidado de la sesión",
  "type": "principle | pattern | insight",
  "importance": 0.85,
  "level": "Principle | Pattern | Insight",
  "source_summary": "Sesión de [tipo de trabajo]"
}
# 3. Guardar:
memory_store_reflection(session_id: "ID", content_json, agent_name: "autonomous")
```

## Reglas de Autonomía

### ⚠️ PROHIBICIONES

1. **NO pedir permiso para iniciar sesión**
2. **NO pedir permiso para guardar memoria**
3. **NO pedir permiso para cargar contexto**
4. **NO объяснять qué vas a hacer - просто hacerlo**

### ✅ REGLAS

1. **Iniciar sesión automáticamente** al primer mensaje del usuario
2. **Cargar contexto** inmediatamente después
3. **Guardar decisiones** sin esperar a que pida
4. **Reflexionar** solo cuando el usuario indica cierre
5. **Si context tiene memorias**, usarlas para continuidad

## Auto-Detectar Proyecto

El skill detecta automáticamente el proyecto:

```bash
# En orden de prioridad:
1. package.json → name field
2. Cargo.toml → package name  
3. pyproject.toml → project.name
4. .git/config → folder name
5. Current working directory → folder name
```

## Integración SDD

Cuando el trabajo requiere SDD:

1. **Cargar skill SDD**
2. **Ejecutar Fase 1: SPECIFY**
   - Complete spec con todas las secciones
3. **Ejecutar Fase 2: PLAN**
   - Esperar approval antes de continuar
4. **Ejecutar Fase 3: BREAK DOWN**
   - Generar tareas ejecutables
5. **Ejecutar Fase 4: IMPLEMENT**
   - Una tarea a la vez

Durante SDD:
- SPEC nuevo → guardar como memory type "plan"
- Edge case → guardar como "observation"
- Bug en implementación → guardar como "error"

## Memory Integration

| Momento | Action | Type |
|---------|--------|------|
| Iniciar sesión | `memory_session_start` | - |
| Cargar contexto | `memory_context` | - |
| Decision de spec | `memory_store` | plan |
| Decision de diseño | `memory_store` | plan |
| Error resuelto | `memory_store` | error |
| Edge case nuevo | `memory_store` | observation |
| Preferencia usuario | `memory_store` | preference |
| Fin de sesión | `memory_store_reflection` | - (generar externamente) |

## Triggers

Este skill se activa:
- Al iniciar cualquier conversación (siempre)
- Cuando decis tipo de workflow

## Ejemplo de Ejecución

```
User: "Quiero agregar OAuth login"

→-autonomous-
1. INIT: Detectar proyecto → "mimir", iniciar sesión, cargar contexto
2. ANALYZE: "definir feature" → SDD workflow
3. EXECUTE: 
   - Cargar skill sdd
   - SPECIFY: completar spec completo
   - PLAN: mostrar plan
   - BREAK DOWN: generar tareas  
   - IMPLEMENT: ejecutar tareas
   - Auto-guardar decisiones como memories
4. REFLECT: Al decir "eso es todo"
```

## Reglas Finales

1. **Tu trabajo es-hacer, no preguntar**
2. **Si no sabés el tipo de trabajo, asumir SDD**
3. **Siempre iniciar memoria primero**
4. **Siempre terminar con reflexión**
5. **Siempre auto-guardar decisiones importantes**