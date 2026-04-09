---
name: architecture-guardrails
description: Límites de arquitectura y ownership. Capas, módulos, dependencias permitidas. Proteger la estructura.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: architecture-safety
---

## ¿Qué soy?

Soy el guardrail de arquitectura. Protejo la estructura del proyecto de cambios que rompan los límites establecidos.

## Capas de Arquitectura

Capas que deben respetarse:

```
┌─────────────────────────────────────┐
│         Routes / Handlers            │  ← NO depende de nada externo
├─────────────────────────────────────┤
│           Services                   │  ← Solo routes
├─────────────────────────────────────┤
│            Models                    │  ← Solo services
├─────────────────────────────────────┤
│          Database                    │  ← Solo models
├─────────────────────────────────────┤
│        External APIs                 │  ← Solo configuración
└─────────────────────────────────────┘
```

**Regla:** Cada capa solo puede depender de la capa inmediata debajo.

## Ownership de Módulos

| Módulo | Dueño | Responsabilidad |
|--------|-------|------------------|
| `auth/*` | Auth service | Login, register, tokens |
| `api/*` | API team | Endpoints, validación |
| `db/*` | DB team | Models, migrations |
| `ui/*` | Frontend team | Componentes, UX |

## Dependencias Permitidas

###allowed (verde):
- Dependencias dentro del mismo módulo
- Dependencias de capas inferiores

###⚠️ BLOCKED (rojo):
- Saltar capas (UI → DB directo)
- Dependencias circulares
- Módulos no relacionados

## Errores Comunes

| Error | Problema | Solución |
|-------|----------|---------|
| Circular dependency | A → B → C → A | Refactorear |
| Layer violation | UI → DB | Usar servicio intermedio |
| Feature envy | Módulo usa demasiado de otro | Mover lógica |
| God module | Un archivo hace todo | Dividir |

## Capas por Proyecto

### Rust Project
```
src/
├── main.rs
├── routes/          → Handlers HTTP
├── services/       → Lógica de negocio
├── models/         → Tipos y entidades
├── db/             → DB access
└── external/       → APIs externas
```

### TypeScript Project
```
src/
├── routes/          → Express routes
├── controllers/    → Lógica
├── services/       → Negocio
├── models/         → Types/Mongo
├── db/            → Mongo connection
└── api/            → External APIs
```

## Reglas de Arquitectura

### ✅ REGlas

1. **Cada archivo tiene una responsabilidad**
2. **Dependencias fluyen hacia abajo**
3. **No hay dependencias circulares**
4. **Módulos tienen fronteras claras**
5. **Cambios dentro de límites**

### ⚠️ PROHIBICIONES

1. **NO crear god files (1000+ líneas)**
2. **NO saltarse capas**
3. **NO importar directo DB en routes**
4. **NO bypassear servicios**
5. **NO crear dependencas circulares**

## Verificar Arquitectura

```bash
# Ver dependencias
cargo-deps  # Rust
npx madge   # Node

# Ver estructura
find src -type f | head -20

# Ver tamaño de archivos
wc -l src/**/*.rs
```

## Checklist Pre-Commit

- [ ] No hay dependencias circulares
- [ ] No se saltan capas
- [ ] Cada archivo < 500 líneas
- [ ] Feature en módulo correcto
- [ ] No hay god files

## Module Boundaries

```
┌──────────────────────────────────────┐
│  BOUNDARY                            │
│  ┌────────────────────────────┐    │
│  │  Module A            │    │
│  │  - public_api.rs    │    │  
│  │  - internal.rs     │    │  ← internals NO cruzables
│  └────────────────────────────┘    │
│  ┌────────────────────────────┐    │
│  │  Module B            │    │
│  │  - public_api.rs    │    │
│  │  - internal.rs     │    │
│  └────────────────────────────┘    │
└──────────────────────────────────────┘
```

## Triggers

Este skill se activa cuando:
- Se va a crear nuevo archivo
- Se va a modificar imports
- Se va a cambiar estructura
- Se menciona "architecture", "module", "dependency"

## Autonomía

Como siempre, pregunto antes de cambios estructurales:

1. **Antes de crear archivo** → Te pregunto en qué módulo
2. **Antes de nuevo import** → Verifico que no rompa capas
3. **Antes de restructure** → Te muestro el plan
4. **Si hay violacion** → Te lo señalo y sugiero alternativa

## Regla de Hierro

**"Si tenés que preguntar dónde va, preguntá antes de crear."**

No creo archivos sin tu approval del lugar.

---

## Ejemplo de Detection

```
⚠️ ARQUITECTURE VIOLATION: 

Módulo intentando: UI → DB directo

┌─────────────┐
│ routes.rs  │ → importando db.rs ❌
└─────────────┘
     ↓
┌─────────────┐
│ services.rs │ → importando db.rs ✅
└─────────────┘
     ↓
┌─────────────┐
│   db.rs    │
└─────────────┘

Sugerencia: Usar services como intermediaria
```