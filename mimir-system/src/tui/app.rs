//! Simple TUI menu with interactive input

use crate::db::{get_connection, list_sessions};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Get projects
    let projects = get_projects();
    let mut selected = 0;

    // Main loop
    while RUNNING.load(Ordering::SeqCst) {
        clear_screen(&mut stdout);
        draw_menu(&projects, selected);
        stdout.flush()?;

        // Wait for input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        RUNNING.store(false, Ordering::SeqCst);
                    }
                    KeyCode::Char('j') | KeyCode::Down | KeyCode::Char('k') | KeyCode::Up => {
                        if key.code == KeyCode::Char('j') || key.code == KeyCode::Down {
                            selected = (selected + 1).min(projects.len().saturating_sub(1));
                        } else {
                            selected = selected.saturating_sub(1);
                        }
                    }
                    KeyCode::Char('1') => {
                        selected = 0;
                        handle_selection(&projects, selected);
                    }
                    KeyCode::Char('2')
                    | KeyCode::Char('3')
                    | KeyCode::Char('4')
                    | KeyCode::Char('5') => {
                        let idx = key.code.to_string().parse::<usize>().unwrap_or(1) - 1;
                        if idx < projects.len() {
                            selected = idx;
                            handle_selection(&projects, selected);
                        }
                    }
                    KeyCode::Enter => {
                        handle_selection(&projects, selected);
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode().ok();
    execute!(stdout, LeaveAlternateScreen).ok();
    print!("\n");

    Ok(())
}

fn get_projects() -> Vec<String> {
    if let Ok(conn) = get_connection() {
        if let Ok(sessions) = list_sessions(&conn, "") {
            let mut p: Vec<String> = sessions.into_iter().map(|s| s.project).collect();
            p.sort();
            p.dedup();
            return p;
        }
    }
    vec![]
}

fn clear_screen(stdout: &mut io::Stdout) {
    println!("\x1b[2J\x1b[H");
}

fn draw_menu(projects: &[String], selected: usize) {
    println!("╔════════════════════════════════════════╗");
    println!("║        🧠 MIMIR TUI v0.1.0         ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  Seleccioná una opción:               ║");
    println!("║                                     ║");
    println!("║  📁 1. Proyectos                  ║");
    println!("║  📋 2. Sesiones                   ║");
    println!("║  💾 3. Memorias                  ║");
    println!("║  🔍 4. Buscar                    ║");
    println!("║  ❓ 5. Ayuda                     ║");
    println!("║                                     ║");
    println!("║  ⬆️⬇️  Navegar con j/k              ║");
    println!("║  ⏎     Seleccionar               ║");
    println!("║  q     Salir                      ║");
    println!("╚════════════════════════════════════════╝");

    if !projects.is_empty() {
        println!("\n📁 Proyectos ({}):", projects.len());
        for (i, p) in projects.iter().take(10).enumerate() {
            let marker = if i == selected { "▶" } else { " " };
            println!("  {} {}", marker, p);
        }
    }

    print!("\n> ");
}

fn handle_selection(projects: &[String], selected: usize) {
    // Placeholder for actual navigation
    if let Some(p) = projects.get(selected) {
        println!("\n✅ Seleccionaste: {}\n", p);
    }
}
