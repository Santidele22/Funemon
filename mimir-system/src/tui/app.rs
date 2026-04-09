//! Simple TUI menu with interactive input

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
    let mut selected = 0;
    let mut view = 0; // 0=menu, 1=projects, 2=sessions, 3=memories, 4=search, 5=help

    while RUNNING.load(Ordering::SeqCst) {
        println!("\x1b[2J\x1b[H");

        match view {
            0 => draw_main_menu(&projects, &mut selected),
            1 => draw_projects_view(&projects, &mut selected),
            2 => draw_sessions_view(),
            3 => draw_memories_view(),
            4 => draw_search_view(),
            5 => draw_help_view(),
            _ => {}
        }

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => RUNNING.store(false, Ordering::SeqCst),
                    KeyCode::Esc => view = 0,
                    KeyCode::Char('j') | KeyCode::Down => {
                        let max = get_max(view, &projects);
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
    println!("\n");
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

fn get_max(view: usize, projects: &[String]) -> usize {
    match view {
        0 => 4,
        1 => projects.len().saturating_sub(1),
        2 => 4,
        3 => 4,
        4 => 0,
        5 => 0,
        _ => 0,
    }
}

fn draw_main_menu(projects: &[String], selected: &mut usize) {
    println!("╔════════════════════════════════════════╗");
    println!("║        🧠 MIMIR TUI v0.1.0           ║");
    println!("╠════════════════════════════════════════╣");

    let items = [
        "📁 Proyectos",
        "📋 Sesiones",
        "💾 Memorias",
        "🔍 Buscar",
        "❓ Ayuda",
    ];
    for (i, item) in items.iter().enumerate() {
        let marker = if *selected == i { "▶" } else { " " };
        println!("║  {} {}. {}                   ║", marker, i + 1, item);
    }

    println!("╠════════════════════════════════════════╣");
    println!("║  j/k: navegar  |  Enter: entrar     ║");
    println!("║  Esc: inicio  |  q: salir         ║");
    println!("╚════════════════════════════════════════╝");

    if !projects.is_empty() {
        println!("\n📁 Tus proyectos ({}):", projects.len());
        for p in projects.iter().take(8) {
            println!("   {}", p);
        }
    }
    print!("\n> ");
}

fn draw_projects_view(projects: &[String], selected: &mut usize) {
    println!("╔════════════════════════════════════════╗");
    println!("║        📁 PROYECTOS                 ║");
    println!("╠════════════════════════════════════════╣");

    if projects.is_empty() {
        println!("║  No hay proyectos aún               ║");
    } else {
        for (i, p) in projects.iter().take(10).enumerate() {
            let marker = if *selected == i { "▶" } else { " " };
            let name = if p.len() > 28 { &p[..28] } else { p.as_str() };
            println!("║  {} {:.<28}║", marker, name);
        }
    }

    println!("╠════════════════════════════════════════╣");
    println!("║  j/k: navegar  |  Enter: seleccionar║");
    println!("║  Esc: volver |  q: salir         ║");
    println!("╚════════════════════════════════════════╝");
    print!("\n> ");
}

fn draw_sessions_view() {
    println!("╔════════════════════════════════════════╗");
    println!("║        📋 SESIONES                 ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  Selecciona un proyecto primero     ║");
    println!("║  desde la vista de Proyectos     ║");
    println!("╚════════════════════════════════════════╝");
    print!("\n> ");
}

fn draw_memories_view() {
    println!("╔════════════════════════════════════════╗");
    println!("║        💾 MEMORIAS                  ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  Selecciona una sesión primero    ║");
    println!("║  desde la vista de Sesiones    ║");
    println!("╚════════════════════════════════════════╝");
    print!("\n> ");
}

fn draw_search_view() {
    println!("╔════════════════════════════════════════╗");
    println!("║        🔍 BUSCAR                  ║");
    println!("╠════════════════════════════════════════���");
    println!("║  Escribe para buscar en memorias     ║");
    println!("╚════════════════════════════════════════╝");
    print!("\n> ");
}

fn draw_help_view() {
    println!("╔════════════════════════════════════════╗");
    println!("║        ❓ AYUDA                     ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  COMANDOS:                          ║");
    println!("║  1-5: Ir a vista                   ║");
    println!("║  j/k: navegar                       ║");
    println!("║  Enter: entrar/seleccionar          ║");
    println!("║  Esc: volver al menú                 ║");
    println!("║  q: salir                        ║");
    println!("╚════════════════════════════════════════╝");
    print!("\n> ");
}
