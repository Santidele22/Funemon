//! Simple TUI menu

use crate::db::{get_connection, list_sessions};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let projects = get_projects();
    let names = get_projects_names();
    let mut selected = 0;
    let mut view = 0;

    while RUNNING.load(Ordering::SeqCst) {
        println!("\x1b[2J\x1b[H");

        match view {
            0 => draw_main_menu(&names, selected),
            1 => draw_projects(&projects),
            2 => draw_sessions(&projects, selected, &names),
            3 => draw_memories(),
            4 => draw_search(),
            5 => draw_help(),
            _ => {}
        }

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => RUNNING.store(false, Ordering::SeqCst),
                    KeyCode::Esc => {
                        view = 0;
                        selected = 0;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        let max = match view {
                            0 => 4,
                            _ => 10,
                        };
                        if selected < max {
                            selected += 1;
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Char('1') => {
                        view = 1;
                        selected = 0;
                    }
                    KeyCode::Char('2') => {
                        view = 2;
                        selected = 0;
                    }
                    KeyCode::Char('3') => {
                        view = 3;
                        selected = 0;
                    }
                    KeyCode::Char('4') => {
                        view = 4;
                        selected = 0;
                    }
                    KeyCode::Char('5') => {
                        view = 5;
                        selected = 0;
                    }
                    KeyCode::Enter => {
                        if view == 0 {
                            view = selected + 1;
                            selected = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().ok();
    execute!(stdout, LeaveAlternateScreen).ok();
    println!();
    Ok(())
}

fn get_projects() -> Vec<(String, String)> {
    if let Ok(conn) = get_connection() {
        if let Ok(sessions) = list_sessions(&conn, "") {
            let mut result: Vec<(String, String)> = sessions
                .into_iter()
                .map(|s| {
                    let date = chrono::DateTime::from_timestamp(s.last_active, 0);
                    let date_str = match date {
                        Some(d) => d.format("%Y-%m-%d").to_string(),
                        None => "?".to_string(),
                    };
                    (s.project, date_str)
                })
                .collect();
            result.sort_by(|a, b| b.1.cmp(&a.1));
            return result;
        }
    }
    vec![]
}

fn get_projects_names() -> Vec<String> {
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

fn draw_main_menu(names: &[String], selected: usize) {
    println!("🧠 MIMIR TUI v0.1.0");
    println!();

    let items = ["Proyectos", "Sesiones", "Memorias", "Buscar", "Ayuda"];
    for (i, item) in items.iter().enumerate() {
        let marker = if selected == i { "▶" } else { " " };
        println!("{} {}. {}", marker, i + 1, item);
    }

    println!();
    println!("j/k: navegar | Enter: entrar | Esc: inicio | q: salir");

    if !names.is_empty() {
        println!();
        println!("📁 Tus proyectos ({}):", names.len());
        for p in names.iter().take(8) {
            println!("   - {}", p);
        }
    }
    print!("\n> ");
}

fn draw_projects(projects: &[(String, String)]) {
    println!("📁 PROYECTOS");
    println!();

    if projects.is_empty() {
        println!("No hay proyectos");
    } else {
        let mut grouped: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (p, d) in projects {
            if !grouped.contains_key(p) {
                grouped.insert(p.clone(), d.clone());
            }
        }

        let mut list: Vec<_> = grouped.into_iter().collect();
        list.sort_by(|a, b| b.1.cmp(&a.1));

        for (i, (p, d)) in list.iter().take(12).enumerate() {
            let marker = if i == 0 { "▶" } else { " " };
            println!("{} {} ({})", marker, p, d);
        }
    }

    println!();
    print!("\n> ");
}

fn draw_sessions(projects: &[(String, String)], selected: usize, names: &[String]) {
    println!("📋 SESIONES");
    println!();

    if let Some(pname) = names.get(selected) {
        println!("Proyecto: {}", pname);
        println!();

        let count = projects.iter().filter(|(p, _)| p == pname).count();
        println!("Total de sesiones: {}", count);

        if count > 0 {
            let session_dates: Vec<_> = projects
                .iter()
                .filter(|(p, _)| p == pname)
                .map(|(_, d)| d.clone())
                .collect();
            for d in session_dates.iter().take(5) {
                println!("  - {}", d);
            }
            if count > 5 {
                println!("  ... y {} mas", count - 5);
            }
        } else {
            println!("No hay sesiones");
        }
    } else {
        println!("Selecciona un proyecto");
    }

    println!();
    print!("\n> ");
}

fn draw_memories() {
    println!("💾 MEMORIAS");
    println!();

    println!("Selecciona una sesion para ver memorias");
    println!();
    println!("Ve a Sesiones (2) para seleccionar");
    print!("\n> ");
}

fn draw_search() {
    println!("🔍 BUSCAR");
    println!();
    println!("Proximamente...");
    print!("\n> ");
}

fn draw_help() {
    println!("❓ AYUDA");
    println!();
    println!("COMANDOS:");
    println!("  1-5: Ir a seccion");
    println!("  j/k: navegar");
    println!("  Enter: entrar");
    println!("  Esc: volver");
    println!("  q: salir");
    print!("\n> ");
}
