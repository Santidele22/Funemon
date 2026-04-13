# Troubleshooting - Funemon

Este documento contiene soluciones a problemas comunes durante la instalación y uso de Funemon, el sistema de memoria persistente.

## 🦀 Rust Compatibility Issues

### Error: "feature `edition2024` is required"

**Síntoma:**
```
error: failed to parse manifest at `.../clap_lex-1.1.0/Cargo.toml`
Caused by: feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.75.0).
```

**Causa:**
Algunas dependencias como `clap_lex v1.1.0` y `anstyle-query v1.1.5` requieren `edition2024` que solo está disponible en Rust 1.85+. Si tienes Rust 1.75 o anterior, la compilación fallará.

**Solución:**

#### Opción 1: Actualizar Rust (Recomendado)

```bash
# Actualizar Rust a la última versión estable
rustup update stable

# Verificar la versión
rustc --version
# Debería mostrar 1.85.0 o superior
```

#### Opción 2: El fix ya está en main

Si ya están aplicados los fixes en el repositorio, las dependencias problemáticas han sido ajustadas:

- `clap_lex = "0.6"` (en lugar de versiones más nuevas que requieren edition2024)
- `anstyle-query` downgradeado a v1.0.0 en Cargo.lock

**Si todavía tienes problemas después de actualizar:**

```bash
# Limpiar caché de Cargo
cargo clean
rm -rf Cargo.lock

# Reconstruir
cargo build --release
```

## 📦 Problemas de Build

### Error de dependencias

```bash
# Limpiar y reconstruir
cargo clean
cargo build --release
```

### OpenSSL Not Found

Si ves el error:
```
Could not find openssl via pkg-config:
Package 'openssl', required by 'virtual:world', not found
```

**Causa:**
El sistema no tiene instalados los paquetes de desarrollo de OpenSSL necesarios para compilar dependencias como `native-tls` o `openssl-sys`.

**Solución:**

Instala los paquetes de desarrollo de OpenSSL según tu sistema operativo:

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel pkg-config

# macOS
brew install openssl pkg-config

# Arch Linux
sudo pacman -S openssl pkg-config
```

Después de instalar, vuelve a compilar:

```bash
cargo build --release
```

**Nota para macOS:**
Si después de instalar siguen habiendo problemas, configura las variables de entorno:
```bash
export OPENSSL_DIR=/usr/local/opt/openssl
export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
```

### Error de SQLite

```bash
# Ubuntu/Debian
sudo apt-get install -y libsqlite3-dev

# macOS
brew install sqlite3
```

## 🔧 Problemas de Instalación

### Error al instalar el binario

**Síntoma:** `cargo install --path funemon-system` falla.

**Solución:**
1. Verifica que tienes Rust 1.75+:
   ```bash
   rustc --version
   ```

2. Verifica dependencias del sistema:
   ```bash
   # Linux
   sudo apt-get install -y build-essential pkg-config libssl-dev libsqlite3-dev
   
   # macOS
   brew install openssl sqlite3
   ```

3. Limpia y reconstruye:
   ```bash
   cargo clean
   cargo build --release
   cargo install --path funemon-system
   ```

## 🧠 Problemas de Memoria

### El comando `funemon init` falla

**Síntoma:** Error al inicializar la base de datos.

**Solución:**
1. Verifica permisos del directorio:
   ```bash
   ls -la ~/.local/share/funemon/
   ```

2. Crea el directorio si no existe:
   ```bash
   mkdir -p ~/.local/share/funemon
   ```

3. Ejecuta init con verbose:
   ```bash
   FUNEMON_LOG=debug funemon init
   ```

### Las memorias no se guardan

**Síntoma:** Los comandos de memoria no persisten.

**Diagnóstico:**
```bash
# Verificar que la BD existe
ls -la ~/.local/share/funemon/funemon.db

# Verificar permisos
stat ~/.local/share/funemon/funemon.db
```

**Solución:**
```bash
# Si la BD no existe, inicializar
funemon init

# Si hay problemas de permisos
chmod 644 ~/.local/share/funemon/funemon.db
```

### Error: "database is locked"

**Síntoma:** SQLite reporta que la base de datos está bloqueada.

**Solución:**
1. Verifica procesos usando Funemon:
   ```bash
   ps aux | grep funemon
   ```

2. Mata procesos bloqueados:
   ```bash
   pkill -f funemon
   ```

3. Reinicia el servidor:
   ```bash
   funemon mcp &
   ```

## 🔌 Problemas de MCP Server

### El servidor MCP no inicia

**Síntoma:** `funemon mcp` falla o no responde.

**Solución:**
1. Verifica el puerto (si usas puerto específico):
   ```bash
   netstat -tlnp | grep funemon
   ```

2. Verifica logs:
   ```bash
   FUNEMON_LOG=debug funemon mcp
   ```

3. Verifica que no hay conflicto:
   ```bash
   pkill -f funemon
   funemon mcp
   ```

### Error de conexión con OpenCode

**Síntoma:** OpenCode no puede conectar con el servidor MCP de Funemon.

**Solución:**
1. Verifica configuración en opencode.json:
   ```bash
   cat ~/.config/opencode/opencode.json
   ```

2. Verifica que el servidor está corriendo:
   ```bash
   pgrep -f funemon
   ```

3. Reinicia ambos:
   ```bash
   pkill -f funemon
   funemon mcp &
   ```

## 📊 Problemas de Rendimiento

### Operaciones lentas

**Síntoma:** Las operaciones de memoria tardan más de 100ms.

**Diagnóstico:**
```bash
# Verificar tamaño de la BD
ls -lh ~/.local/share/funemon/funemon.db

# Ver estadísticas
funemon stats
```

**Solución:**
1. Si la BD es grande (>10MB), considera limpiar:
   ```bash
   funemon cleanup --days 30
   ```

2. Optimiza la BD:
   ```bash
   # Vacío de SQLite
   sqlite3 ~/.local/share/funemon/funemon.db "VACUUM;"
   ```

### Alto uso de memoria

**Síntoma:** Funemon consume mucha memoria RAM.

**Solución:**
1. Limita el número de memorias cargadas:
   ```bash
   # En vez de cargar todas
   funemon context <session-id> --limit 10
   ```

2. Limpia sesiones antiguas:
   ```bash
   funemon cleanup --days 30
   ```

## 🔍 Debug y Logs

### Habilitar logs detallados

```bash
# Nivel debug
export FUNEMON_LOG=debug

# Nivel trace (muy verbose)
export FUNEMON_LOG=trace

# Ejecutar con logs
FUNEMON_LOG=debug funemon mcp
```

### Verificar operaciones SQL

```bash
# SQLite directo
sqlite3 ~/.local/share/funemon/funemon.db

# Ver tablas
.tables

# Ver esquema
.schema

# Contar memorias
SELECT COUNT(*) FROM memories;
SELECT COUNT(*) FROM sessions;
SELECT COUNT(*) FROM reflections;
```

### Health check completo

```bash
#!/bin/bash
echo "=== Funemon Health Check ==="

# Versión
echo "Versión: $(funemon version)"

# BD existe
if [ -f ~/.local/share/funemon/funemon.db ]; then
    echo "✅ Base de datos existe"
    ls -lh ~/.local/share/funemon/funemon.db
else
    echo "❌ Base de datos NO existe"
fi

# Servidor corriendo
if pgrep -f "funemon mcp" > /dev/null; then
    echo "✅ Servidor MCP corriendo"
else
    echo "⚠️  Servidor MCP NO está corriendo"
fi

# Sesiones
echo "Sesiones: $(funemon session list | wc -l)"

# Stats
funemon stats
```

## 📝 Reportar Problemas

Si encuentras un problema no documentado:

1. Busca en [Issues existentes](https://github.com/Santidele22/Funemon/issues)
2. Crea un nuevo issue con:
   - **Versión de Rust**: `rustc --version`
   - **Sistema operativo**: `uname -a`
   - **Output de `funemon stats`**
   - **Logs relevantes**: `FUNEMON_LOG=debug funemon mcp 2>&1`
   - **Pasos para reproducir**

---

**Última actualización:** 2026-04-12