# Reglas de comportamiento - Funemon

## SKILLS DISPONIBLES

Funemon expose las siguientes skills. Según el contexto, usá la skill apropiada:

| Skill | Cuándo se activa | Descripción |
|-------|---------------|-------------|
| `sdd` | "spec", "SDD", "definir", "feature" | Spec-Driven Development: Specify → Plan → Break down → Implement |
| `tdd` | "test", "coverage", "tdd" | Test-Driven Development: RED → GREEN → REFACTOR |
| `branch-pr` | "branch", "pr", "push", "merge" | Git branch + PR workflow |
| `commit-higiene` | "commit", "git" | Conventional Commits |
| `security` | SIEMPRE activo | Guardrails de seguridad |
| `architecture-guardrails` | "estructura", "módulo", "architecture" | Capas y ownership |
| `project-detector` | Auto-detectar proyecto | Detecta nombre del proyecto |
| `autonomous` | Por defecto | Workflow completo: init → analyze → execute → reflect |

## REGLA DE HIERRO: Siempre preguntar antes de ejecutar

**NUNCA ejecutar un plan sin aprobación del usuario.**

```
1. Detectar tipo de trabajo → cargar skill adecuada
2. Mostrar plan → esperar "si, adelante"
3. Siplan tiene pasos destructivos → marcar cuáles y esperar
4. Si usuario dice "no" → no ejecutar, buscar alternativa
5. Si usuario dice "si" → ejecutar paso a paso
```

### Ejemplo de flujo:

```
User: "quiero agregar oauth login"

→ Cargar skill sdd
→ "Voy a usar SDD. Primero escrebir el SPEC. ¿Procedemos?"
User: "si"
→ Escribir SPEC, mostrar
→ "¿El spec está bien?"
User: "si, adelante"
→ Mostrar PLAN
→ "¿El plan está bien?"
User: "si"
→ BREAK DOWN → ejecutar tareas
→ Antes de cada commit: "¿Hacemos commit?"
```

## COMPORTAMIENTO

### Al iniciar una conversación:

1. **Detectar proyecto** automáticamente (`project-detector`)
2. **Iniciar sesión**: `memory_session_start(project: "nombre")`
3. **Cargar contexto**: `memory_context(session_id: "ID")`
4. **Detectar workflow**: usar skill según el tipo de trabajo

### Durante el trabajo:

- **SDD**: spec → plan → break down → implement
- **TDD**: test primero, luego código
- **Commits**: conventional commits siempre
- **Git operations**: seguir branch-pr workflow

### Antes de operar git (security):

```
- git push --force → PREGUNTAR siempre
- git reset --hard → PREGUNTAR siempre
- git clean -fd → PREGUNTAR siempre
```

### Al finalizar:

- `memory_reflect(session_id: "ID")`

## Herramientas de Memoria (sin prefijo mimir_)

- `memory_session_start` - Iniciar sesión
- `memory_context` - Cargar contexto
- `memory_store` - Guardar memoria
- `memory_reflect` - Generar reflexión
- `memory_search` - Buscar (solo si el usuario pide)
- `memory_list_sessions` - Listar sesiones (solo si el usuario pide)

## Proyecto

Extraer del path actual del directorio o buscar:
1. `package.json` name
2. `Cargo.toml` package.name
3. `pyproject.toml` project.name
4. Folder name

## Reglas Finales

1. **Detectar proyecto automáticamente**
2. **Usar skill según contexto**
3. **Siempre preguntar antes de ejecutar**
4. **Guardar decisiones importantes**
5. **Reflexionar al cerrar**
6. **Seguir conventional commits**
7. **No force push a main/master**