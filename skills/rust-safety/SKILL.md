---
name: rust-safety
description: Rust safety guide - lifetimes, borrowing, thread safety, async. Reglas para escribir código seguro en Rust.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: rust-safety
---

## ¿Qué soy?

Soy el guardián de seguridad en Rust. Te ayudo a evitar errores comunes con lifetimes, borrowing, y thread safety.

## Reglas de Borrowing

### ✅ Reglas de Oro

1. **Solo UN mutable OR varios inmutables** - nunca ambos
2. **References siempre válidas** - nunca dangling
3. **Referencias viven menos que los datos** - borrow checker verifica

### Errores Comunes

| Error | Causa | Solución |
|-------|------|----------|
| `cannot borrow as mutable` | Already borrowed immutable | Usar solo una referencia mutable |
| `borrow of moved value` | Valor movido | Clonar o usar referencia |
| `lifetime mismatch` | Reference más corta que dato | Definir lifetimes |

## Lifetimes

### Sintaxis

```rust
fn foo<'a>(x: &'a str) -> &'a str {
    x  // 'a vive al menos como x
}
```

### Regla

- Return reference debe tener lifetime de input
- No crear lifetimes innecesarias
- Usar `'static` solo cuando realmente necesaria

### Cuando Usar

| Situación | Lifetime |
|----------|---------|
| Referencias en structs | `'a` explícito |
| Datos globales | `'static` |
| Referencias cortas | Inferido |

## Thread Safety

### Send + Sync

```rust
// Para compartir entre threads:
use std::sync::{Arc, Mutex};
use std::thread;

// Arc - reference counted
let data = Arc::new(Mutex::new(0));

// Clone para otro thread
let data2 = Arc::clone(&data);

thread::spawn(move || {
    *data2.lock().unwrap() += 1;
});
```

### Errores Comunes

| Error | Causa | Solución |
|-------|------|----------|
| `cannot send` | Tipo no `Send` | Usar Mutex, channel |
| `cannot be shared` | Comparter sin sincronizar | Usar `Arc<Mutex<T>>` |
| `data race` | Sin sincronización | Siempre con Lock |

### Tabla Thread Safety

| Tipo | Send? | Sync? | Notas |
|------|-------|-------|-------|
| `Arc<T>` | Si T: Send | Si T: Sync | Seguro |
| `Mutex<T>` | Siempre | Siempre | Seguro |
| `Rc<T>` | No | No | Solo single thread |
| `RefCell<T>` | No | No | Solo single thread |
| `Channel` | Si | N/A | Comunica entre threads |

## Async

### Reglas

1. **Executor obligatorio** - async sin executor no corre
2. ** tokio::main** o similar
3. **.await** en funciones async

### Errores Comunes

| Error | Causa | Solución |
|-------|------|----------|
| `cannot use await` | FN no async | Agregar async |
| `future does nothing` | Sin .await | Await todas las Futures |
| `no executor` | Sin Runtime | Usar #[tokio::main] |

### Patrón Correcto

```rust
#[tokio::main]
async fn main() {
    let result = do_async().await;
    println!("{}", result);
}

async fn do_async() -> String {
    // multiple await
    let _ = fetch_data().await;
    let _ = process_data().await;
    "done".to_string()
}
```

## Checklist Pre-Commit

- [ ] No hay mutable borrow con inmutable
- [ ] References tienen lifetimes correctos
- [ ] Datos compartidos usan Arc/Mutex
- [ ] Funciones async usan await
- [ ] No hay datos race potenciales
- [ ] Thread safety verificado

## Triggers

Este skill se activa cuando:
- Trabajamos con código Rust
- Vemos errors de lifetime/borrowing
- Referencias en structs
- Compartimos datos entre threads

## Errores que Detecto

```
⚠️ BORROWING VIOLATION:
  multiple mutable + immutable refs
  
⚠️ LIFETIME MISMATCH:
  reference outlives data
  
⚠️ THREAD SAFETY:
  no Arc<Mutex> for shared data
  
⚠️ ASYNC ISSUE:
  missing await
```

---

## Regla de Hierro

**"Si compilás, no significa que está correcto. Verificá lifetimes y thread safety."**