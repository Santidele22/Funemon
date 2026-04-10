# Skills Disponibles - Funemon

> *Las skills son capacidades especializadas que los agentes pueden invocar para tareas específicas.*

## ¿Qué son las Skills?

Las skills son flujos de trabajo predefinidos que encapsulan conocimiento y mejores prácticas para tareas específicas. En lugar de repetir instrucciones, los agentes simplemente llaman a la skill apropiada.

## Skills Activas

| Skill | Propósito | Trigger |
|-------|-----------|---------|
| **reflection** | Generar reflexiones de sesión | Final de sesión o explicito |
| **sdd** | Spec-Driven Development | "spec", "feature", "definir" |
| **tdd** | Test-Driven Development | "test", "coverage", "tdd" |
| **branch-pr** | Git branch + PR workflow | "branch", "pr", "push" |
| **commit-higiene** | Conventional Commits | "commit", "git commit" |
| **security** | Guardrails de seguridad | Siempre activo |
| **architecture-guardrails** | Validar arquitectura | "estructura", "módulo" |
| **project-detector** | Auto-detectar proyecto | Al iniciar sesión |
| **autonomous** | Workflow completo | Por defecto |

---

## Skill: `reflection`

### Descripción

Genera una reflexión inteligente sobre el trabajo realizado en una sesión. Analiza las memorias guardadas y produce un resumen de alto nivel con:

- Temas principales trabajados
- Decisiones tomadas
- Lecciones aprendidas
- Patrones detectados
- Sugerencias para futuras sesiones

### Cuándo Usarla

- **Automático**: Al finalizar una sesión de trabajo
- **Manual**: Cuando el usuario pide un resumen
- **Post-feature**: Después de implementar una feature importante
- **Post-bugfix**: Después de resolver un bug complejo
- **Post-review**: Después de una revisión de código

### Parámetros

| Parámetro | Tipo | Requerido | Default | Descripción |
|-----------|------|-----------|---------|-------------|
| `agent_name` | string | No | "tyrion" | Nombre del agente que reflexiona |

### Ejemplos de Uso

#### Ejemplo 1: Reflexión automática al final de sesión

```bash
# Desde CLI
funemon skill reflection

# Desde MCP (llamada de agente)
skill_reflection()
```

#### Ejemplo 2: Tyrion reflexiona después de implementar feature

```yaml
# Contexto: Tyrion acaba de implementar autenticación OAuth

Agente: "Implementé OAuth login. Todo funciona. Veo si hay reflexión..."

→ Llama: skill_reflection(agent_name: "tyrion")

Output:
## Reflexión de Sesión - Tyrion

### Trabajo Realizado
- Implementé feature de OAuth login
- Agregué 3 providers: Google, GitHub, Auth0
- Tests: 94% coverage

### Decisiones Clave
- Use PKCE flow por seguridad (vs implicit flow)
- Prefiero state storage in Redis (vs JWT sessions)
- Timestamps en UTC siempre

### Lecciones Aprendidas
- PKCE evita token leakage en public clients
- Redis state es más simple que JWT para OAuth

### Patrones Detectados
- Auth-then-redirect pattern en todos los providers
- Token refresh automático cada 55 minutos

### Para Próxima Sesión
- Considerar rate limiting en /auth/callback
- Agregar circuit breaker para OAuth APIs
```

#### Ejemplo 3: Magnus reflexiona después de tests

```yaml
# Contexto: Magnus terminó de implementar una skill

Agente: "Terminé la skill de reflexiones. Tests pasando."

→ Llama: skill_reflection(agent_name: "magnus")

Output:
## Reflexión de Sesión - Magnus

### Trabajo Realizado
- Implementé skill_reflection como skill autónoma
- Refactoricé reflection_engine.rs
- Agregué 15 tests nuevos

### Decisiones Clave
- Skill usa interface estándar (vs custom)
- Parámetro agent_name opcional con default
- Reflexiones se guardan en memoria para contexto futuro

### Lecciones Aprendidas
- Skills son más flexibles que funciones internas
- Default parameters mejoran UX para agentes

### Para Próxima Sesión
- Documentar skill en SKILLS.md
- Considerar skill "memory-cleanup" similar
```

#### Ejemplo 4: Bruno reflexiona después de debugging

```yaml
# Contexto: Bruno encontró y fixeó un bug difícil

Agente: "Encontré el bug en el reflection engine..."

→ Llama: skill_reflection(agent_name: "bruno")

Output:
## Reflexión de Sesión - Bruno

### Bug Resuelto
- Bug crítico: reflexiones duplicadas en DB
- Root cause: race condition en SQLite
- Fix: agregué mutex en write path

### Debugging Path
1. Reproduje localmente (10 intentos)
2. Log tracing mostró timing issue
3. SQLite no maneja bien concurrent writes
4. Solución: lock explícito

### Lecciones Aprendidas
- SQLite necesita mutex en write-heavy scenarios
- Race conditions son sleeping dragons
- Always log con timestamps presión

### Para Próxima Sesión
- Considerar migrar a PostgreSQL para production
- Agregar integration tests de concurrencia
```

### Output Format

La reflexión genera un documento Markdown estructurado:

```markdown
## Reflexión de Sesión - {agent_name}

### Trabajo Realizado
- [Lista de tareas completadas]

### Decisiones Clave
- [Decisiones tomadas con contexto]

### Lecciones Aprendidas
- [Insights del trabajo]

### Patrones Detectados
- [Patrones de código o comportamiento]

### Para Próxima Sesión
- [Sugerencias y próximos pasos]
```

### Integración con Memoria

Las reflexiones se guardan automáticamente en la memoria del proyecto:

```sql
-- Tabla: reflections
session_id |reflection_content | created_at
-----------|-------------------|------------uuid | "# Reflexión..." | 2024-01-15...
```

Las reflexiones previas se cargan automáticamente en `memory_context`:

```bash
# Al iniciar sesión, elagente ve reflexiones previas
→ Reflexión anterior: "Implementé OAuth con PKCE..."
→ Decisión previa: "Prefiero Redis para state storage"
```

---

## Skill: `sdd` (Spec-Driven Development)

### Descripción

Workflow completo de desarrollo guiado por especificaciones.

### Fases

1. **Specify**: definir comportamiento esperado
2. **Plan**: diseñar arquitectura
3. **Break down**: dividir en tareas
4. **Implement**: ejecutar tareas

### Cuándo Usarla

- Nueva feature importante
- Refactoring grande
- Cambios de arquitectura

---

## Skill: `tdd` (Test-Driven Development)

### Descripción

Ciclo RED → GREEN → REFACTOR con automation.

### Fases

1. **RED**: escribir test que falla
2. **GREEN**: código mínimo para pasar
3. **REFACTOR**: limpiar mientras tests pasan

### Cuándo Usarla

- Features con lógica compleja
- Bug fixes con regression tests
- Code coverage improvements

---

## Skill: `branch-pr` (Branch & PR Workflow)

### Descripción

Workflow de Git con branches y Pull Requests.

### Pasos

1. Crear branch descriptivo
2. Commits atómicos
3. Push con -u flag
4. Crear PR con descripción completa
5. Esperar review

### Cuándo Usarla

- Feature development
- Bug fixes
- Refactoring

---

## Skill: `commit-higiene` (Conventional Commits)

### Descripción

Estructura de commits siguiendo conventional commits.

### Formato

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types válidos

- `feat`: nueva feature
- `fix`: bug fix
- `docs`: documentación
- `refactor`: refactoring
- `test`: tests
- `chore`: mantenimiento

---

## Skill: `security`

### Descripción

Guardrails de seguridad Siempre activo.

### Reglas

- NO hardcode secrets
- NO force push a main/master
- NO ejecutar comandos destructivos sin permiso
- NO commits con archivos sensibles

---

## Skill: `architecture-guardrails`

### Descripción

Valida arquitectura y ownership.

### Reglas

- Un módulo = una responsabilidad
- Dependencias unidireccionales
- No cyclic dependencies

---

## Skill: `project-detector`

### Descripción

Detecta automáticamente el nombre del proyecto.

### Prioridad

1. `package.json` → name
2. `Cargo.toml` → package.name
3. `pyproject.toml` → project.name
4. Folder name

---

## Skill: `autonomous`

### Descripción

Workflow completo autónomo.

### Fases

1. **Init**: detectar proyecto + iniciar sesión
2. **Analyze**: cargar contexto + detectar tipo de trabajo
3. **Execute**: usar skill apropiada + guardar memorias
4. **Reflect**: generar reflexión al finalizar

---

## Cómo Crear Nuevas Skills

Las skills se definen en `/skills/` con la siguiente estructura:

```
skills/
├── reflection/
│   ├── skill.md # Instrucciones
│   ├── skill.json # Metadata│   └── examples/
│       └── example1.md
```

### skill.json

```json
{
 "name": "reflection",
  "description": "Genera reflexiones de sesión",
  "trigger": ["reflection", "reflect", "resumen"],
  "parameters": [
    {
      "name": "agent_name",
      "type": "string",
      "required": false,
      "default": "tyrion"
    }
  ]
}
```

### skill.md

Instrucciones detalladas para el agente enMarkdown.

---

## Índice de Skills por Agente

| Agente | Skills Principales |
|--------|-------------------|
| **Tyrion** | `sdd`, `tdd`, `branch-pr` |
| **Magnus** | `autonomous`, `commit-higiene` |
| **Bruno** | `security`, `architecture-guardrails` |
| **Almendra** | `reflection`, `project-detector` |

---

*Documentación generada por Almendra - La documentalista*