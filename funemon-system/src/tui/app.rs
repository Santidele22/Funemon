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

const TITLE: &str = "FUNEMON";
const WIDTH: usize = 60;

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
    box_title();
    println!();

    let items = ["Proyectos", "Sesiones", "Memorias", "Buscar", "Ayuda"];
    for (i, item) in items.iter().enumerate() {
        let marker = if selected == i { "▶" } else { " " };
        box_line(&format!("{}. {}", i + 1, item), false);
    }

    box_line(
        "j/k: navegar | Enter: entrar | Esc: inicio | q: salir",
        false,
    );

    if !names.is_empty() {
        println!();
        box_line(&format!("Tus proyectos ({}):", names.len()), false);
        for p in names.iter().take(8) {
            box_line(&format!("  - {}", p), false);
        }
    }

    println!("{}", get_right_border());
    print!("> ");
}

fn draw_projects(projects: &[(String, String)]) {
    box_title();

    if projects.is_empty() {
        box_line("No hay proyectos", false);
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
            box_line(&format!("{} {} ({})", marker, p, d), false);
        }
    }

    println!("{}", get_right_border());
    print!("> ");
}

fn draw_sessions(projects: &[(String, String)], selected: usize, names: &[String]) {
    box_title();

    if let Some(pname) = names.get(selected) {
        box_line(&format!("Proyecto: {}", pname), false);

        let count = projects.iter().filter(|(p, _)| p == pname).count();
        box_line(&format!("Total de sesiones: {}", count), false);

        if count > 0 {
            let session_dates: Vec<_> = projects
                .iter()
                .filter(|(p, _)| p == pname)
                .map(|(_, d)| d.clone())
                .collect();
            for d in session_dates.iter().take(5) {
                box_line(&format!("  - {}", d), false);
            }
            if count > 5 {
                box_line(&format!("  ... y {} mas", count - 5), false);
            }
        } else {
            box_line("No hay sesiones", false);
        }
    } else {
        box_line("Selecciona un proyecto", false);
    }

    println!("{}", get_right_border());
    print!("> ");
}

fn draw_memories() {
    box_title();
    box_line("Selecciona una sesion para ver memorias", false);
    box_line("Ve a Sesiones (2) para seleccionar", false);
    println!("{}", get_right_border());
    print!("> ");
}

fn draw_search() {
    box_title();
    box_line("Proximamente...", false);
    println!("{}", get_right_border());
    print!("> ");
}

fn draw_help() {
    box_title();
    box_line("COMANDOS:", false);
    box_line("  1-5: Ir a seccion", false);
    box_line("  j/k: navegar", false);
    box_line("  Enter: entrar", false);
    box_line("  Esc: volver", false);
    box_line("  q: salir", false);
    println!("{}", get_right_border());
    print!("> ");
}

// ============ BOX DRAWING FUNCTIONS ============

fn box_title() {
    let title = format!(" 🧠 {} ", TITLE);
    let padding = WIDTH.saturating_sub(title.len());
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;

    println!(
        "┌{}{}{}┐",
        "─".repeat(left_pad),
        title,
        "─".repeat(right_pad)
    );
}

fn box_line(content: &str, is_last: bool) {
    let len = content.len();
    if len > WIDTH - 2 {
        // Truncate content if too long
        let truncated = &content[..WIDTH - 5];
        println!("│ {} │", truncated);
    } else {
        println!("│ {}{} │", content, " ".repeat(WIDTH - 2 - len));
    }
}

fn get_right_border() -> String {
    "└".to_string() + &"─".repeat(WIDTH) + "┘"
}
