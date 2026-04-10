---
name: rust-cli
description: Rust CLI patterns con clap. Commands, subcommands, argument parsing, validacion. Guide para crear CLIs robustos.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: rust-cli
---

## ¿Qué soy?

Soy el guide de CLI en Rust. Te ayudo a crear interfaces de línea de comandos robustas con clap.

## Estructura Básica

### Imports

```rust
use clap::{Parser, Subcommand, ValueEnum};
```

### Parser Basic

```rust
#[derive(Parser)]
#[command(name = "mi-cli")]
#[command(about = "Descripcion breve", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
```

### Subcommands

```rust
#[derive(Subcommand)]
pub enum Commands {
    /// Descripcion del comando
    Start {
        /// Nombre del proyecto
        #[arg(short, long)]
        project: String,
    },
    /// Inicializar
    Init,
    /// Ver status
    Status {
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}
```

### Custom Types

```rust
#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Parser)]
pub struct ConfigCmd {
    #[arg(short, long, default_value = "json")]
    format: OutputFormat,
}
```

## Validadores

### Custom Validation

```rust
#[derive(Parser)]
pub struct StartCmd {
    #[arg(short, long)]
    name: String,
    
    // Validacion: no vacio
    #[arg(long, validator = validate_name)]
    project: Option<String>,
}

fn validate_name(s: &str) -> Result<(), String> {
    if s.is_empty() {
        Err("Name cannot be empty".to_string())
    } else if s.contains(' ') {
        Err("Name cannot contain spaces".to_string())
    } else {
        Ok(())
    }
}
```

### Range Validation

```rust
#[arg(long, value_parser = clap::value_parser!(u16).range(1..))]
port: u16,
```

## Command Handlers

### Async Handler

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start(cmd) => handle_start(cmd).await?,
        Commands::Init => handle_init()?,
        Commands::Status(cmd) => handle_status(cmd)?,
    }
    
    Ok(())
}
```

### Error Handling

```rust
fn handle_start(cmd: StartCmd) -> Result<(), Box<dyn std::error::Error>> {
    match start_project(&cmd.name) {
        Ok(id) => println!("Started: {}", id),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
```

## Best Practices

### ✅ SIEMPRE

- Usar `#[derive(Parser)]` para CLI
- Usar subcommands paraCommands related
- Validar inputs con validators
- Help messages con `/// Docs`
- Usar tipos custom para enums
- Manejar errores explicitamente

### ✅ NUNCA

- No usar `unwrap()` sin reason
- No ignorar errores
- No hardcodear constants
- No Skip validation

### ✅ Naming

| Command | Subcommand | Args |
|---------|------------|------|
| `start` | `project` | `--name` |
| `init` | - | - |
| `list` | `projects` | `--filter` |
| `delete` | `session` | `--id` |

## Ejemplo Completo

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "funemon")]
#[command(about = "Sistema de memoria", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Inicia nueva sesion
    Start {
        /// Nombre del proyecto
        #[arg(short, long)]
        project: String,
    },
    /// Lista sesiones
    List {
        /// Filtrar por proyecto
        #[arg(short, long)]
        project: Option<String>,
    },
    /// Inicializa la base de datos
    Init,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start { project } => {
            println!("Starting: {}", project);
        }
        Commands::List { project } => {
            println!("Listing: {:?}", project);
        }
        Commands::Init => {
            println!("Initializing...");
        }
    }
    
    Ok(())
}
```

## Checklist

- [ ] Estructura con derive(Parser)
- [ ] Subcommands paraCommands relacionados
- [ ] Documentacion con ///
- [ ] Validation donde needed
- [ ] Error handling apropiado
- [ ] Async main si async commands

## Triggers

Este skill se activa cuando:
- Creamos CLI con clap
- Commands y subcommands
- Argument parsing
- Validacion de inputs

---

## Regla de Hierro

**"CLI bien diseñada se explica sola. Si necesitás explicar, está mal diseñada."**