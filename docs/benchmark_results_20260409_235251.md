# Benchmark Results - OpenCode Ecosystem

**Date:** 2026-04-09 23:52:51

## Test 1: Funemon CLI Solo

| Operation | Time (ms) |
|-----------|-----------|
| Startup | 216 |
| Write (avg of 10) | 26,9 |
| Context | 15 |
| Search | 20 |

## Test 2: Funemon MCP Tools

| Operation | Time (ms) |
|-----------|-----------|
| Startup | 58 |
| Write (avg of 10) | 12 |
| Context | 12 |

## Test 3: Reflexions

| Operation | Time (ms) |
|-----------|-----------|
| Store | 14 |
| Get | 14 |

## Test 4: Resource Usage

| Metric | Value |
|--------|-------|
| Binary size | 6,6M |
| Database size | 4,0K |
| Total memories | 0
0 |
| Total sessions | 0
0 |

---

## Summary

### Performance Comparison

| Test | Funemon CLI | Funemon MCP |
|------|-------------|-------------|
| Startup | 216ms | 58ms |
| Write (avg) | 26,9ms | 12ms |
| Context | 15ms | 12ms |
| Search | 20ms | N/A |

### Notes

- **OpenCode Solo**: No tiene persistencia de memoria. Cada sesión comienza desde cero.
- **OpenCode + Funemon**: Usa tools MCP para persistir contexto entre sesiones.
- **Funemon CLI**: Usa directamente el CLI sin intermediarios.
- **OpenCode + Agentes**: Los agentes (Magnus, Aurora, Bruno) usan Funemon automáticamente.

