---
name: sdd
description: Spec-Driven Development - 4 fases obligatorias: Specify → Plan → Break down → Implement. Nunca escribir código hasta completar Specify y Plan.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: spec-driven-development
---

## ¿Qué soy?

Soy el workflow SDD (Spec-Driven Development). Exijo que definas el SPEC antes de escribir código. El spec es la verdad, el código es descartable.

## Las 4 Fases (OBLIGATORIAS)

### Fase 1: SPECIFY (primera)

**NUNCA pasar a otra fase hasta completar esta.**

Describe qué vas a construir y POR QUÉ:
- User stories
- Acceptance criteria específicos
- Edge cases (qué pasa cuando falla, input inválido, API down, etc.)
- Constraints técnicos

Formato:
```
## SPEC: [nombre del feature]

### Problem Statement
[Qué problema resuelve - una oración]

### User Stories
- Como [usuario], quiero [acción] para [beneficio]
- Como [usuario], quiero [acción] para [beneficio]

### Acceptance Criteria
- [ ] [Criterio medible #1]
- [ ] [Criterio medible #2]

### Edge Cases
- [Qué pasa cuando X]
- [Qué pasa cuando Y]

### Constraints
- [Stack: Rust/TypeScript/etc]
- [Patrones requeridos]
- [Dependencias existentes]
```

### Fase 2: PLAN (segunda)

**NUNCA escribir código hasta aprobar este plan.**

Define la implementación técnica:
- Arquitectura
- Estructura de archivos
- Service boundaries
- Decisiones técnicas

Formato:
```
## PLAN: [nombre del feature]

### Architecture
[Diagrama o texto de cómo encaja en el sistema]

### Files to Create/Modify
- `src/foo.rs` - nueva funcionalidad
- `tests/foo_test.rs` - tests
- `docs/foo.md` - documentación

### Dependencies
- [Nueva dependencia]
- [Dependencia existente]

### Implementation Notes
[Puntos técnicos a considerar]
```

### Fase 3: BREAK DOWN (tercera)

**Convierte el plan en tareas ejecutables.**

Cada tarea tiene:
- Input
- Expected output
- Criterios de validación

Formato:
```
## TASK BREAKDOWN

### Task 1: [nombre]
- Input: [qué recibe]
- Output: [qué produce]
- Validation: [cómo verificás que funciona]

### Task 2: [nombre]
- Input: [qué recibe]
- Output: [qué produce]
- Validation: [cómo verificás que funciona]

### Dependencies
- [Task 1] → [Task 2] (secuencial)
- [Task 3] puede correr en paralelo
```

### Fase 4: IMPLEMENT (cuarta)

**Ejecutar las tareas en orden.**

Reglas:
- Usar SPEC y PLAN como contexto para CADA decisión
- Si te desviás, actualizar SPEC, no el código
- Tests primero (TDD) donde aplique

## Reglas de SDD

### ⚠️ PROHIBICIONES

1. **NO escribir código en Fase 1 o 2**
2. **NO pasar a Implement sin Plan aprobado**
3. **NO modificar código sin actualizar spec si cambió el requerimiento**
4. **NO decir "escribí código" hasta estar en Fase 4**

### ✅ REGLAS

1. **El spec es la verdad, el código es descartable** - se puede regenerar
2. **Código nuevo = spec nuevo primero**
3. **Tests verifican el spec, no el código**
4. **Edge cases van en spec, no son "sorpresas"**

## Integración con Memory

Al iniciar SDD:
1. `memory_session_start(project)` 
2. `memory_context(session_id)` 

Durante SDD:
- Al crear spec → guardar como type: "plan" (es un plan!)
- Al encontrar edge case → guardar como type: "observation"
- Al resolver bug → guardar como type: "error"

Al cerrar SDD:
1. Generar reflexión analizando las memorias de la sesión
2. Estructurar como JSON: `{content, type, importance, level, source_summary}`
3. Llamar `memory_store_reflection(session_id, content_json, agent_name)`

## Triggers

Este skill se activa cuando el usuario menciona:
- "spec" / "specification" / "especificación"
- "SDD" / "spec-driven"
- "plan" seguido de feature nuevo
- "definir" + "feature" / "requerimiento"
- "cómo va" + "arquitectura" / "diseño"

## Workflow Completo

```
User: "Quiero agregar autenticación por OAuth"

→ Fase 1 SPECIFY:
  Escribir spec completo
  
→ Fase 2 PLAN:
  Mostrar plan, esperar approval
  
→ Fase 3 BREAK DOWN:
  Generar tareas
  
→ Fase 4 IMPLEMENT:
  Ejecutar tareas una por una
```

## Ejemplo

```
## SPEC: Login OAuth GitHub

### Problem Statement
Usuarios pueden autenticarse con su cuenta GitHub

### User Stories
- Como usuario, quiero hacer login con GitHub
- Como usuario, no quiero crear contraseña nueva

### Acceptance Criteria
- [ ] Botón "Login with GitHub" visible en login page
- [ ] Redirect a OAuth GitHub
- [ ] Callback guarda usuario en DB
- [ ] Session creada después de OAuth

### Edge Cases
- [ ] Usuario rechaza permisos → volver a login con error
- [ ] Token expire → reintentar automáticamente
- [ ] Usuario ya existe → hacer update, no duplicate

### Constraints
- Stack: Rust + Axum
- OAuth provider: github
- Session: cookie con HttpOnly
```

---

## Reglas de autonomia SDD

1. **NUNCA escribir código antes de completar SPEC** 
2. **NUNCA escribir código antes de completar PLAN**
3. **Si el usuario pide código inline, pedir spec primero**
4. **Si el usuario dice "escribí algo rápido", advertír que SDD requiere spec**
5. **Si el usuario cambia de idea, actualizar spec, no código**