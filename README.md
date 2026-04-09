# Mimir - Sistema de Memoria Persistente para Agentes de Programación

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.94.1-orange" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-Apache--2.0-blue" alt="License">
  <img src="https://img.shields.io/badge/MCP-Protocol-blue" alt="MCP Protocol">
</p>

Mimir es un sistema de memoria persistente diseñado para agentes de programación IA. Mantiene contexto entre sesiones, guarda decisiones, errores y preferencias, y genera reflexiones automáticas.

## Características

- **Memoria Persistente**: Guarda información entre sesiones de trabajo
- **MCP Server**: Implementa el Model Context Protocol para integración con agentes IA
- **Búsqueda Full-Text**: Búsqueda rápida de memorias usando SQLite FTS
- **Reflexiones Automáticas**: Genera resúmenes inteligente de cada sesión
- **Tipos de Memoria**: error, plan, observation, preference
- **Integración OpenCode**: Configuración lista para usar con OpenCode

## Instalación

### Requisitos

- Rust 1.80+
- SQLite
- Ollama (para reflexiones)

### Build

```bash
cd mimir-system
cargo build --release
```

### Instalación del binario

```bash
cargo install --path mimir-system
```

## Configuración

### OpenCode

Copia `opencode.json` a `~/.config/opencode/opencode.json`:

```bash
cp mimir-system/opencode.json ~/.config/opencode/opencode.json
```

### Inicializar Base de Datos

```bash
mimir init
```

## Uso

### CLI

```bash
# Iniciar una sesión
mimir session start --project "mi-proyecto"

# Listar sesiones
mimir session list

# Guardar una memoria
mimir memories store --session-id "uuid" --title "Error resuelto" --type "error" --what "Descripción del error" --why "Cómo se resolvió"

# Buscar memorias
mimir memories search --session-id "uuid" "búsqueda"

# Generar reflexión
mimir reflection generate --session-id "uuid"

# Ver estadísticas
mimir stats
```

### MCP Server

```bash
# Iniciar servidor MCP
mimir mcp
```

El servidor MCP expone las siguientes tools:

- `memory_session_start` - Iniciar/reanudar sesión
- `memory_context` - Cargar contexto de sesión
- `memory_store` - Guardar memoria
- `memory_reflect` - Generar reflexión
- `memory_search` - Buscar memorias
- `memory_list_sessions` - Listar sesiones

## Configuración de OpenCode

Ver `opencode.json` para la configuración completa. El agente usará las tools de memoria de forma autónoma:

1. Al iniciar: `memory_session_start` + `memory_context`
2. Durante el trabajo: guardar errores, planes, observaciones, preferencias
3. Al finalizar: `memory_reflect`

## Estructura del Proyecto

```
Mimir/
├── mimir-system/
│   ├── src/
│   │   ├── cli/          # Interfaz CLI
│   │   ├── db/           # Base de datos SQLite
│   │   ├── mcp/          # Servidor MCP
│   │   └── reflection/   # Generación de reflexiones
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
- **reqwest**: HTTP client (para Ollama)

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