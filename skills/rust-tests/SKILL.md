---
name: rust-tests
description: Rust testing patterns. Unit tests, integration tests, mocking, test databases. Guide para tests en Rust.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: rust-tests
---

## ¿Qué soy?

Soy el guide de testing en Rust. Te ayudo a escribir tests efectivos.

## Estructura de Tests

### Module de Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Tests en Archivos Separados

```
src/
├── lib.rs
└── lib_test.rs  // alternative convention
```

## Unit Tests

### Test Functions

```rust
#[test]
fn test_add() {
    let result = Calculator::add(2, 3);
    assert_eq!(result, 5);
}

#[test]
fn test_subtract() {
    let result = Calculator::subtract(5, 3);
    assert_eq!(result, 2);
}
```

### Test Struct

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_user() {
        let user = User::new("test");
        assert_eq!(user.name, "test");
        assert!(user.id.is_some());
    }
    
    #[test]
    fn user_default() {
        let user = User::default();
        assert_eq!(user.name, "default");
    }
}
```

## Assertions

### Basic

```rust
assert!(condition);
assert_eq!(a, b);
assert_ne!(a, b);
```

### With Messages

```rust
assert!(result.is_ok(), "Should be Ok");
assert_eq!(value, expected, "custom message");
```

### Float

```rust
use float_cmp::approx_eq;

#[test]
fn test_float() {
    let a = 0.1 + 0.2;
    assert!(approx_eq!(f64, a, 0.3, epsilon = 0.0001));
}
```

## Integration Tests

### In tests/ Directory

```rust
// tests/integration_test.rs

use mimir::*;

#[test]
fn test_full_flow() {
    let result = do_something();
    assert!(result.is_ok());
}
```

### Database Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        init_schema(&conn)?;
        Ok(conn)
    }
    
    #[test]
    fn test_insert_user() {
        let conn = test_db().unwrap();
        let user = User::new("test");
        user.save(&conn).unwrap();
        
        let fetched = User::get(&conn, &user.id).unwrap();
        assert_eq!(fetched.name, "test");
    }
}
```

## Mocking

### Custom Mocks

```rust
struct MockService {
    response: String,
}

impl Service for MockService {
    fn fetch(&self) -> Result<String> {
        Ok(self.response.clone())
    }
}

#[test]
fn test_with_mock() {
    let mock = MockService { response: "mocked".to_string() };
    let result = process(&mock);
    assert_eq!(result, "mocked");
}
```

### with mockall

```rust
#[cfg(test)]
mod tests {
    use mockall::mock;
    
    mock! {
        Database {
            fn insert(&self, user: &User) -> Result<()>;
            fn get(&self, id: &str) -> Result<Option<User>>;
        }
    }
    
    #[test]
    fn test_mock() {
        let mut db = MockDatabase::new();
        db.expect_insert()
            .returning(|_| Ok(()));
            
        db.insert(&user).unwrap();
    }
}
```

## Async Tests

### with tokio

```rust
#[tokio::test]
async fn test_async() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Multiples

```rust
#[tokio::test]
async fn test_parallel() {
    let h1 = tokio::spawn(async { do_something().await });
    let h2 = tokio::spawn(async { do_other().await });
    
    h1.await.unwrap().unwrap();
    h2.await.unwrap().unwrap();
}
```

## Test Organization

### Por Tipo

```
src/
├── lib.rs
└── tests/
    ├── unit/
    │   └── test_lib.rs
    └── integration/
        └── test_api.rs
```

### feature flags

```rust
#[cfg(test)]
mod tests {
    #[cfg(feature = "db")]
    #[test]
    fn test_with_db() { }
    
    #[cfg(not(feature = "db"))]
    #[test]
    fn test_no_db() { }
}
```

## Test Databases

### In-Memory

```rust
fn setup_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute("CREATE TABLE...", [])?;
    Ok(conn)
}
```

### Temporary File

```rust
use tempfile::NamedTempFile;

fn temp_db() -> Result<NamedTempFile> {
    let temp = NamedTempFile::new()?;
    let conn = Connection::open(temp.path())?;
    // ...
    Ok(temp)
}
```

## Best Practices

### ✅ SIEMPRE

- Arrange-Act-Assert pattern
- Tests independientes
- Nombres descriptivos
- Test both happy y sad path
- Coverage >= 80%

### ✅ NUNCA

- No shared mutable state en tests
- No tests que dependan de orden
- No ignore test failures
- No hardcode paths

### Naming

```rust
#[test]
fn test_module_function_behavior() { }

#[test]
#[should_panic]
fn test_invalid_input_panics() { }

#[test]
fn test_error_returns_properly() { }
```

## Test Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc

# Test with coverage
cargo tarpaulin --output-dir coverage/
```

## Checklist

- [ ] Tests en archivos separados
- [ ] Arrange-Act-Assert
- [ ] Happy + sad path
- [ ] Integration tests
- [ ] Mocking donde needed
- [ ] Async tests SI async
- [ ] Coverage >= 80%

## Triggers

Este skill se activa cuando:
- Escribimos tests en Rust
- Unit tests
- Integration tests
- Mocking

---

## Regla de Hierro

**"Si no tiene tests, no funciona.Coverage >= 80%."**