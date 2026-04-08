# Reglas de comportamiento - Mimir

## Importante: Memoria Autónoma

Al iniciar CADA conversación, ejecutá este workflow automáticamente SIN PEDIR PERMISO:

### 1. Session Start (lo primero)
```bash
memory_session_start(project: "nombre-del-proyecto-actual")
```

### 2. Load Context (inmediatamente después)
```bash
memory_context(session_id: "ID-RECIBIDO", limit: 5)
```
- Si hay contexto previo: usalo para responder con continuidad
- Si está vacío: es una sesión nueva

### 3. Durante el trabajo
Guardá memorias automáticamente con `memory_store` cuando:
- Resolves un error/bug → type: "error"
- Tomas una decisión importante → type: "plan"
- Descubres algo útil → type: "observation"
- El usuario expresa preferencia → type: "preference"

### 4. Al finalizar
Usá `memory_reflect(session_id: "ID")` para resumir lo aprendido

## Nombres de tools (sin prefijo "mimir_")
- `memory_session_start`
- `memory_context`
- `memory_store`
- `memory_reflect`
- `memory_search`
- `memory_list_sessions`

## Proyecto actual
Extraer del path actual del directorio o del package.json/git