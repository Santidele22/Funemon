---
name: tdd
description: Test-Driven Development - Tests primero, coverage obligatorio. Red-green-refactor. No code sin tests.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: test-driven-development
---

## ¿Qué soy?

Soy el workflow TDD. Exijo que escribas los tests ANTES de escribir código. No hay excepción: si no hay test, no hay código.

## Las 3 Leyes del TDD

1. **NO escribir código de producción** hasta tener un test que falla
2. **NO escribir más test** del necesario para que falle
3. **NO escribir más código** del necesario para que pase el test

## El Ciclo Red-Green-Refactor

```
    ┌──────────┐
    │  RED    │  ← Escribir test que FALLA
    └────┬────┘
         │
         ▼
    ┌──────────┐
    │  GREEN  │  ← Escribir mínimo código para que PASE
    └────┬────┘
         │
         ▼
    └──────────┐
    │ REFACTOR│  ← Optimizar sin cambiar comportamiento
    └──────────┘
```

## Coverage Requerido

| Tipo de Código | Coverage Mínimo |
|----------------|-----------------|
| **Nueva funcionalidad** | 80% |
| **Bug fix** | 90% |
| **Critical path** | 100% |
| **Lógica de negocio** | 90% |

**Regla:** Ningún PR sin coverage >= 80%

## Estructura de Tests

### Para cada feature/nuevo archivo:

```
tests/
├── unit/
│   └── [module]_test.rs
├── integration/
│   └── [feature]_test.rs
└── e2e/
    └── [flow]_test.rs
```

## workflow de TDD

### Paso 1: Escribir el Test (RED)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_feature_does_expected() {
        // Arrange
        let input = setup_input();
        
        // Act (esto va a fallar porque no existe la función)
        let result = module::new_feature(input);
        
        // Assert
        assert_eq!(result, expected_output());
    }
}
```

### Paso 2: Escribir Código Mínimo (GREEN)

Escribir solo lo necesario para que pase el test. No más.

### Paso 3: Refactorizar (REFACTOR)

Limpiar el código sin cambiar comportamiento. Luego:
- Ejecutar tests
- Verificar coverage
- Si coverage baja → agregar tests

### Paso 4: Verificar

```bash
# ejecuto tests
cargo test

# verifico coverage
cargo tarpaulin --output-dir reports/

# si coverage < 80% → agregar más tests
```

## Tipos de Tests

| Test | Cuándo | Ejemplo |
|------|--------|---------|
| **Unit** | Lógica individual | `fn calculate_total() - 10% discount = 90` |
| **Integration** | Múltiples módulos | `POST /api/users → 201 Created` |
| **E2E** | Flujo completo | `User signup → login → dashboard` |
| **Property** | Prop invariantes | `sort(vec) → is_sorted(vec)` |

## Reglas de TDD

### ⚠️ PROHIBICIONES

1. **NO escribir código sin test anterior**
2. **NO escribir test después del código**
3. **NO hacer "test coverage" después (cover your tracks)**
4. **NO pushear sin tests pasando**

### ✅ REGLAS

1. **Test primero, siempre**
2. **Mínimo código para pasar test**
3. **Refactor después, no antes**
4. **Coverage >= 80% obligatorio**
5. **Todos los tests pasan antes de PR**

## Test Naming Convention

Usar nombre descriptivo:

```
test_[what]_[expected_behavior]
test_calculate_total_with_discount_returns_reduced_price
test_user_login_with_invalid_token_returns_error
test_api_create_user_without_email_returns_validation_error
```

## Ejecutar Tests

```bash
# Unit tests
cargo test

# With coverage
cargo tarpaunit --output-dir coverage/

# Specific test
cargo test test_name

# Watch mode (dev)
cargo watch -x test
```

## Integración con SDD

SDD + TDD = Stack completo:

| Fase SDD | Fase TDD |
|---------|----------|
| SPECIFY | → Escribir tests en BREAK DOWN |
| PLAN | → Definir archivos de test |
| BREAK DOWN | → Tasks incluyen tests |
| IMPLEMENT | → RED → GREEN → REFACTOR |

## Checklist Pre-Push

- [ ] Todos los tests pasan (`cargo test`)
- [ ] Coverage >= 80%
- [ ] No hay tests comentados
- [ ] Tests nombrados correctamente
- [ ] Tests son independientes (no dependen de orden)

## Triggers

Este skill se activa cuando:
- Se va a escribir código nuevo
- Se va a modificar código existente
- Se va a hacer bug fix
- Se menciona "test", "tdd", "coverage", "pytest"

## Reglas de Autonomía

 Como siempre preguntamos antes de ejecutar:

1. **Antes de escribir código** → Te muestro el test que voy a escribir
2. ** Esper tu confirmación** → Vos decis si el test está bien
3. **Escribo código mínimo** → Muestro el resultado
4. **Refactorizo** → Te muestro cambios antes de commit

**Antes de cada push/pre-PR:**
- Ejecutar tests
- Mostrar coverage
- Esperar tu OK para proceder