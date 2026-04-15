# Reglas de comportamiento - Funemon

## SKILLS DISPONIBLES

Funemon expose las siguientes skills. Según el contexto, usá la skill apropiada:

| Skill | Cuándo se activa | Descripción |
|-------|---------------|-------------|
| `sdd` | "spec", "SDD", "definir", "feature" | Spec-Driven Development: Specify → Plan → Break down → Implement |
| `tdd` | "test", "coverage", "tdd" | Test-Driven Development: RED → GREEN → REFACTOR |
| `reflection` | Final de sesión, explicito | Genera reflexión de la sesión |
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

### ANTES de iniciar sesión:

**IMPORTANTE**: Detectar proyecto automáticamente ANTES de llamar a memory_session_start

1. Ejecutar skill `project-detector` para obtener nombre del proyecto
2. Si el skill detecta el proyecto, usar ese nombre
3. Si el skill falla, pedir al usuario que especifique el proyecto
4. NUNCA usar nombres genéricos como "santi-home" o "home-user"

**Ejemplo correcto**:
```
# Detectar proyecto
skill("project-detector") → detecta "funemon-ecosystem"
# Iniciar sesión con nombre detectado
funemon_memory_session_start(project: "funemon-ecosystem")
```

### Al iniciar una conversación:

1. **Detectar proyecto** automáticamente (`project-detector`)
2. **Iniciar sesión**: `funemon_memory_session_start(project: "nombre")`
3. **Cargar contexto**: `funemon_memory_context(session_id: "ID")`
4. **Cargar contexto del proyecto**: `funemon_memory_project_context(project: "nombre", limit: 10)`
5. **Detectar workflow**: usar skill según el tipo de trabajo

### Nueva tool: funemon_memory_project_context

**CUÁNDO usar**: Al inicio de cada sesión para cargar contexto del proyecto

**Uso**:
```bash
funemon_memory_project_context(
  project: "nombre-del-proyecto",
  limit: 10  # opcional, default 10
)
```

**Diferencia con memory_context**:
- `memory_context(session_id)` → solo memorias de ESTA sesión
- `memory_project_context(project, limit)` → memorias de TODAS las sesiones del proyecto

**Ejemplo**:
```bash
# Inicio de sesión
session_id = funemon_memory_session_start(project: "funemon-ecosystem")# Cargar contexto del proyecto (no solo esta sesión)
project_memories = funemon_memory_project_context("funemon-ecosystem", limit: 10)

# Ahora tengo contexto de trabajo anterior
```

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

**Generar reflexión** (el agente reflexiona, NO funemon):

```
1. El agente analiza el trabajo realizado usando su propio LLM
2. Genera reflexión en formato JSON con esta estructura:
   - content: Resumen consolidado de la sesión
   - type: "pattern" | "principle" | "warning"
   - importance: 0.0 a 1.0 (decimal)
   - level: "Fact" | "Pattern" | "Principle"
   - source_summary: Breve descripción de la sesión
3. Guarda la reflexión: `memory_store_reflection(session_id, content_json, agent_name)`
```

**Ejemplo de reflexión generada por el agente (JSON):**

```json
{
  "content": "Implementé OAuth login con 3 providers. Usé PKCE por seguridad. Agregué tests con 94% coverage.",
  "type": "principle",
  "importance": 0.85,
  "level": "Principle",
  "source_summary": "Sesión de desarrollo de autenticación"
}
```

**Guardado:**
```bash
funemon_memory_store_reflection(
  session_id: "uuid",
  content: '{"content": "...", "type": "principle", "importance": 0.85, "level": "Principle", "source_summary": "..."}',
  agent_name: "tyrion"
)
```
1. El agente analiza el trabajo realizado usando su propio LLM
2. Genera reflexión en formato Markdown:
   - Trabajo realizado
   - Decisiones clave
   - Lecciones aprendidas
   - Para próxima sesión
3. Guarda la reflexión: `memory_store_reflection(session_id, content, agent_name)`
```

**Ejemplo de reflexión generada por el agente:**

```markdown
## Reflexión de Sesión - Tyrion

### Trabajo Realizado
- Implementé OAuth login con 3 providers
- Agregú tests: 94% coverage

### Decisiones Clave
- Usé PKCE por seguridad (vs implicit flow)
- Prefiero Redis para state storage

### Lecciones Aprendidas
- PKCE evita token leakage en public clients

### Para Próxima Sesión
- Agregar rate limiting en /auth/callback
```

**Guardado:**
```bash
funemon_memory_store_reflection(
  session_id: "uuid",
  content: "# Reflexión...",
  agent_name: "tyrion"
)
```

## Herramientas de Memoria (con prefijo funemon_)

**Gestión de Sesiones:**
- `funemon_memory_session_start` - Iniciar sesión
- `funemon_memory_context` - Cargar contexto de sesión actual
- `funemon_memory_project_context` - Cargar contexto de todo el proyecto (NUEVO)
- `funemon_memory_list_sessions` - Listar sesiones (solo si el usuario pide)

**Gestión de Memorias:**
- `funemon_memory_store` - Guardar memoria (error, plan, observation, preference)
- `funemon_memory_search` - Buscar memorias (solo si el usuario pide)

**Gestión de Reflexiones:**
- `funemon_memory_store_reflection` - Guardar reflexión generada por el agente
- `funemon_memory_get_reflection` - Obtener reflexión de una sesión

**Limpieza:**
- `funemon_memory_delete_session` - Eliminar sesión (solo si el usuario pide)
- `funemon_memory_cleanup` - Limpiar sesiones inactivas (solo si el usuario pide)

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

## Reglas de Git y Merge (PARA TYRION)

### Regla Obligatoria para TODOS los merges:
1. **NUNCA** pushear a `main` directamente
2. **SIEMPRE** crear rama feature: `git checkout -b feat/descripcion`
3. Trabajar en la rama
4. **SIEMPRE** generar PR: `gh pr create`
5. **PEDIR PERMISO**: "¿Apruebas?" ANTES de merge
6. **Solo mergear con tu aprobación explícita**
7. **NUNCA** hacer merge sin tu OK