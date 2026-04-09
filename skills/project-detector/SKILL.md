---
name: project-detector
description: Auto-detectar proyecto del directorio actual. Lee package.json, Cargo.toml, pyproject.toml, .git/config para extraer nombre del proyecto.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: project-detection
---

## ¿Qué soy?

Soy el detector de proyectos. Automatically determino el nombre del proyecto actual buscando archivos de configuración comunes.

## Cómo detectar

### Orden de prioridad

```
1. package.json → name field
2. Cargo.toml → package name  
3. pyproject.toml → project.name
4. pyproject.toml → tool.poetry.name
5. go.mod → module name (sin el /v2 etc)
6. .git/config → [remote "origin"] url → repo name
7. Current working directory → folder name
```

### Algoritmo

```bash
# 1. Buscar package.json
IF exists("package.json"):
  READ file
  EXTRACT "name" field
  RETURN name

# 2. Buscar Cargo.toml  
ELSE IF exists("Cargo.toml"):
  READ file
  EXTRACT "package.name" field
  RETURN name

# 3. Buscar pyproject.toml
ELSE IF exists("pyproject.toml"):
  READ file
  TRY EXTRACT "project.name"
  ELSE TRY EXTRACT "tool.poetry.name"
  RETURN name

# 4. Buscar go.mod
ELSE IF exists("go.mod"):
  READ file
  EXTRACT module from first line
  RETURN module (sin /v2, /v3 etc)

# 5. Buscar .git/config
ELSE IF exists(".git/config"):
  READ file
  EXTRACT repo name from [remote "origin"] url
  RETURN repo-name

# 6. Fallback: folder name
ELSE:
  RETURN current_working_directory_name
```

## Ubicaciones a buscar

| Archivo | Path | Campo a extraer |
|---------|------|---------------|
| `package.json` | `./package.json` | `name` |
| `Cargo.toml` | `./Cargo.toml` | `package.name` |
| `pyproject.toml` | `./pyproject.toml` | `project.name` o `tool.poetry.name` |
| `go.mod` | `./go.mod` | `module` (línea 2) |
| `.git/config` | `./.git/config` | nombre del repo del remote |

## Errores comunes

| Error | Solución |
|-------|----------|
| No encontre ningún archivo | Usar nombre de carpeta |
| package.json sin campo "name" | Buscar alternative o folder |
| Cargo.toml sin [package] | Buscar [workspace] |
| .git/config sin remote | Usar folder name |

## Output

Retorna:
```json
{
  "project": "nombre-del-proyecto",
  "detection_method": "package.json|Cargo.toml|pyproject.toml|go.mod|git|folder",
  "confidence": "high|medium|low"
}
```

## Reglas

1. **SIEMPRE detectar proyecto automáticamente** al iniciar sesión
2. **NO preguntar el nombre del proyecto** - detectarlo
3. **Si noEncontrás archivo, usar folder name**
4. **Guardar método de detección en memoria** (para debugging)

## Triggers

Este skill se activa automáticamente cuando:
- Se llama `memory_session_start` sin project
- El usuario no especifica proyecto
- Se necesita el nombre del proyecto para cualquier operación