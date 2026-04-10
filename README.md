# Funemon - Sistema de Memoria Persistente para Agentes de Programación

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.94.1-orange" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-Apache--2.0-blue" alt="License">
  <img src="https://img.shields.io/badge/MCP-Protocol-blue" alt="MCP Protocol">
</p>

Funemon es un sistema de memoria persistente diseñado para agentes de programación IA. Mantiene contexto entre sesiones, guarda decisiones, errores y preferencias, y almacena reflexiones generadas por los agentes.

## Características

- **Memoria Persistente**: Guarda información entre sesiones de trabajo
- **MCP Server**: Implementa el Model Context Protocol para integración con agentes IA
- **Búsqueda Full-Text**: Búsqueda rápida de memorias usando SQLite FTS
- **Reflexiones**: Almacena reflexiones generadas por agentes externos
- **Tipos de Memoria**: error, plan, observation, preference
- **Integración OpenCode**: Configuración lista para usar con OpenCode

## Instalación

### Requisitos

- Rust 1.80+
- SQLite

### Build

```bash
cd funemon-system
cargo build --release
```

### Instalación del binario

```bash
cargo install --path funemon-system
```

## Configuración

### OpenCode

Copia `opencode.json` a `~/.config/opencode/opencode.json`:

```bash
cp funemon-system/opencode.json ~/.config/opencode/opencode.json
```

### Inicializar Base de Datos

```bash
funemon init
```

## Uso

### CLI

```bash
# Iniciar una sesión
funemon session start --project "mi-proyecto"

# Listar sesiones
funemon session list

# Guardar una memoria
funemon memories store --session-id "uuid" --title "Error resuelto" --type "error" --what "Descripción del error" --why "Cómo se resolvió"

# Buscar memorias
funemon memories search --session-id "uuid" "búsqueda"

# Guardar reflexión (generada por el agente)
funemon reflection store --session-id "uuid" --agent-name "tyrion" --content "Reflexión generada..."

# Ver reflexión de una sesión
funemon reflection get --session-id "uuid"

# Ver estadísticas
funemon stats
```

### MCP Server

```bash
# Iniciar servidor MCP
funemon mcp
```

El servidor MCP expone las siguientes tools:

**Gestión de Sesiones:**
- `memory_session_start` - Iniciar/reanudar sesión
- `memory_context` - Cargar contexto de sesión
- `memory_list_sessions` - Listar sesiones

**Gestión de Memorias:**
- `memory_store` - Guardar memoria (error, plan, observation, preference)
- `memory_search` - Buscar memorias

**Gestión de Reflexiones:**
- `memory_store_reflection` - Guardar reflexión generada por el agente
- `memory_get_reflection` - Obtener reflexión de una sesión

**Limpieza:**
- `memory_delete_session` - Eliminar sesión (soft delete por defecto)
- `memory_cleanup` - Limpiar sesiones inactivas

## Configuración de OpenCode

Ver `opencode.json` para la configuración completa. El agente usará las tools de memoria de forma autónoma:

1. **Al iniciar:**`memory_session_start` → `memory_context`
2. **Durante el trabajo:** Guardar errores, planes, observaciones, preferencias
3. **Al finalizar:** Generar reflexión internay guardar con `memory_store_reflection`

## Estructura del Proyecto

```
Funemon/
├── funemon-system/
│   ├── src/
│   │   ├── cli/          # Interfaz CLI
│ │   ├── db/           # Base de datos SQLite
│ │   ├── mcp/          # Servidor MCP
│   ├── Cargo.toml
│   └── opencode.json     # Configuración OpenCode
└── README.md
```

## Dependencias

- **rmcp**: SDK de Rust para MCP Protocol
- **rusqlite**: SQLite bindings
- **tokio**: Runtime async
- **clap**: CLI parser
- **chrono**: Fechas y tiempos

## Latest Changes

<!-- AUTO_UPDATE_START -->

### Nuevas Features
- TUI interactiva
- Skills para Rust development
- Auto-update de documentación

### Bug Fixes
- Ninguno

<!-- AUTO_UPDATE_END -->

## License

Apache License 2.0 - ver LICENSE file