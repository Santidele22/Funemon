---
name: rust-error-handling
description: Rust error handling - Result pattern, propagation, custom errors. Cómo manejar errores en Rust.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: rust-error-handling
---

## ¿Qué soy?

Soy el guide de error handling en Rust. Uso el patrón Result para manejo de errores robusto.

## Patrón Result

### Estructura básica

```rust
fn do_something() -> Result<T, E> {
    // Success: Ok(value)
    // Failure: Err(error)
}
```

### Errores Personalizados

```rust
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum MyError {
    NotFound(String),
    InvalidInput(String),
    IoError(std::io::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound(s) => write!(f, "Not found: {}", s),
            MyError::InvalidInput(s) => write!(f, "Invalid: {}", s),
            MyError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl StdError for MyError {}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::IoError(e)
    }
}
```

## Propagation

### Operador ?

```rust
fn read_file() -> Result<String, MyError> {
    let content = std::fs::read_to_string("file.txt")?;  // propagates IO error
    Ok(content)
}
```

### Chains

```rust
fn process() -> Result<Data, MyError> {
    let content = read_file()?;
    let parsed = parse(&content)?;
    let validated = validate(&parsed)?;
    Ok(validated)
}
```

## Patrones de Error Handling

### 1. Match Directo

```rust
match do_something() {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}
```

### 2. With unwrap (debug only)

```rust
let value = do_something().unwrap(); // panics on error
```

### 3. expect (debug only)

```rust
let value = do_something().expect("Failed to do something");
```

### 4. unwrap_or (default)

```rust
let value = do_something().unwrap_or(default_value);
```

### 5. unwrap_or_else (lazy)

```rust
let value = do_something().unwrap_or_else(|| {
    eprintln!("Using default");
    default_value
});
```

### 6. map_err

```rust
do_something()
    .map_err(|e| MyError::Custom(e.to_string()))?;
```

### 7. and_then / or_else

```rust
do_something()
    .and_then(|v| process_value(v))
    .or_else(|e| fallback(e))
```

## Errores en Structs

### Con thiserror

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid: {0}")]
    InvalidInput(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Con derive + box

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    msg: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}
```

## Best Practices

### ✅ SIEMPRE

- Usar `Result` para funciones que fallan
- Proveer contexto en errores
- Usar `?` para propagation
- Crear error types específicos

### ✅ NUNCA

- `.unwrap()` en producción
- `panic!()` sin razón
- Ignorar errores con `_`
- Usar `expect()` en producción

### Err vs Option

| Función | Error type | Use case |
|--------|-----------|---------|
| `ok_or()` | None | Dato opcional |
| `ok_or_else()` | None | Fallback lazy |
| `Result<T, E>` | Error | Operación que puede fallar |

## Checklist

- [ ] Funciones que fallan retornan Result
- [ ] Error types tienen Display
- [ ] Propagation con ?
- [ ] Context en errores
- [ ] No unwrap en producción

## Triggers

Este skill se activa cuando:
- Vemos manejo de errores en Rust
- Errores en funciones
- Propagación de errores
- Errores personalizados

---

## Ejemplo Completo

```rust
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = fs::read_to_string("config.json")?;
    let parsed = serde_json::from_str::<Config>(&config)?;
    validate(&parsed)?;
    run(parsed)?;
    Ok(())
}
```

---

## Regla de Hierro

**"Si puede fallar, retorná Result. Si no puede fallar, usá Option."**