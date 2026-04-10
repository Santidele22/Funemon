---
name: debug
description: Workflow estructurado para debugging: reproducir → analizar → guardar → fix → verificar → reflexionar.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: debugging
---

## ¿Qué soy?

Soy el workflow de debugging estructurado. No solo resuelvo el error, entiendo POR QUÉ ocurrió y guardo el aprendizaje.

## El Ciclo de Debugging

```
    ┌──────────────┐
    │   REPRODUCE │  ← Minimizar el error
    └──────┬──────┘
           │
           ▼
    ┌──────────────┐
    │   ANALYZE  │  ← Buscar causa raíz
    └──────┬──────┘
           │
           ▼
    ┌──────────────┐
    │   LEARN    │  ← Guardar en memoria
    └──────┬──────┘
           │
           ▼
    ┌──────────────┐
    │   FIX     │  ← Arreglar el error
    └──────┬──────┘
           │
           ▼
    ┌──────────────┐
    │  VERIFY    │  ← Verificar fix
    └──────┬──────┘
           │
           ▼
    ┌──────────────┐
    │  REFLECT   │  ← Guardar aprendizaje
    └──────────────┘
```

## Fase 1: REPRODUCE

**Objetivo:** Crear el caso más pequeño posible que reproduce el error.

```
Reglas:
- Crear test unitario que falla
- Minimizar código hasta solo lo necesario
- No assumptions - reproducir primero

Ejemplo:
❌ Testing 500 líneas
✅ Testing 5 líneas que fallan
```

### Pasos:

1. Copiar el error exactamente como apareció
2. Ignorar contexto innecesario
3. Crear test mínimo que falla
4. Verificar que falla

## Fase 2: ANALYZE

**Objetivo:** Encontrar la causa raíz, no el síntoma.

```
Reglas:
- Preguntar "por qué" 5 veces
- No asumir, verificar
- Buscar en: código, config, estado, timing, race condition

Técnicas:
- git log --oneline -10
- git diff 
- console.log / prints
- debug / ir a Sumer
- Buscar en código existente
```

### Checklist:

- [ ] Leyó el error completo?
- [ ] Stack trace analizado?
- [ ] Busca en el código?
- [ ] Busca en config?
- [ ] Hay race condition?
- [ ] Probó con código más simple?

## Fase 3: LEARN

**Objetivo:** Guardar en memoria para no repetir.

```
Inmediatamente después de ANALYZE:
memory_store(
  session_id: "ID",
  title: "[tipo de error]: breve descripción",
  type: "error",
  what: "Qué error ocurrió",
  why: "Por qué ocurrió (causa raíz)",
  where_field: "Archivo/línea",
  learned: "Qué aprendiste"
)
```

### Por qué es CRUCIAL:

- Funemon guarda memoria - esto es su strength
- Errores repetidos son los más costosos
- Guardar el patrón evita recidiva

## Fase 4: FIX

**Objetivo:** Arreglar con mínimo cambio.

```
Reglas:
- Mínimo cambio necesario
- No over-engineering
- No features nuevas
- Test sigue fallando? No pasar a VERIFY

Prefiere:
✅ if (x) return x;
❌ if (x === null || x === undefined || x === 0 || x === "" ...)
```

### Código limpio:

```
Arreglar el error, no la app.
Si el código está mal, refactorear después de fix.
```

## Fase 5: VERIFY

**Objetivo:** Verificar que el fix funciona.

```
Pasos:
1. Correr el test que fallaba
2. Correr todos los tests
3. Verificar coverage no bajó
4. Probar manualmente el flow

Si no pasa → volver a ANALYZE
```

## Fase 6: REFLECT

**Objetivo:** Consolidar aprendizaje.

```
Al finalizar debugging:
memory_store(
  session_id: "ID", 
  title: "FIX: [error] resuelto",
  type: "error",
  what: "Error: [descripción]",
  why: "Causa: [por qué ocurría]",
  learned: "Fix: [cómo se resolvió]"
)

# Generar reflexión externamente y guardar:
1. Analizar las memorias de la sesión
2. Estructurar como JSON:
   {
     "content": "Lección aprendida del debugging",
     "type": "pattern | principle | insight",
     "importance": 0.85,
     "level": "Principle | Pattern | Insight",
     "source_summary": "Debug session de [error]"
   }
3. Llamar: memory_store_reflection(session_id, content_json, agent_name)
```

## Errores Comunes

| Error | Problema | Solución |
|-------|----------|---------|
| Fix symtom, not cause | Arreglar lo visible, no lo real | Analizar más profundo |
| Over-engineering | 100 líneas para fix simple | Mínimo cambio |
| No guardar | Olvidar aprendet | Usar memory_store |
| Skip verify | Assume fixes | Siempre test |
| Copy-paste fix | Sin entender | Analizar primero |

## Debugging Techniques

### Rust
```rust
// Debug print
dbg!(&variable);

// Backtrace
std::backtrace::Backtrace::capture();

// Log
tracing::info!("variable: {:?}", var);
```

### JavaScript/TypeScript
```javascript
console.log('var:', var);
console.trace();

// Error boundary
try {} catch (e) { console.error(e); }
```

### Python
```python
import pdb; pdb.set_trace()

# o
print(f"var: {var!r}")
```

## Checklist Debug Completo

- [ ] Error reproducido en test mínimo
- [ ] Causa raíz identificada
- [ ] Guardado en memoria (memory_store)
- [ ] Fix implementado
- [ ] Tests pasando
- [ ] Reflect guardado

## Triggers

Este skill se activa cuando:
- Usuario dice "error", "bug", "no funciona", "falla"
- Stack trace aparece
- Test falla
- Usuario dice "por qué", "por qué ocurre"

## Integración con Memory

| Fase | Action |
|------|--------|
| ANALYZE | memory_store (error info) |
| FIX | - |
| REFLECT | memory_store_reflection (generar externamente) |

## Autonomía

Como siempre, pregunto antes de ejecutar:

1. **Al detectar error** → Te muestro el error
2. **Antes de analizar** → Te muestro mi approach
3. **Antes de fix** → Te muestro el fix propuesto
4. **Antes de commit** → "¿Hacemos commit?"

---

## Regla de Hierro

**"No fix until you understand why."**

Si no sabés por qué falló, no arregles.
Si arreglas sin entender, vas a volver.