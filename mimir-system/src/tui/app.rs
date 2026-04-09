//! Simple TUI menu - stub until ratatui API stabilizes

use crate::db::{get_connection, list_sessions};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::io::Write;

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Get projects
    let projects = if let Ok(conn) = get_connection() {
        if let Ok(sessions) = list_sessions(&conn, "") {
            let mut p: Vec<String> = sessions.into_iter().map(|s| s.project).collect();
            p.sort();
            p.dedup();
            p
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Print menu
    println!("\x1b[2J\x1b[H"); // Clear screen
    println!("╔══════════════════════════════════╗");
    println!("║      🧠 Mimir TUI         ║");
    println!("╠══════════════════════════════════╣");
    println!("║  1. Projects                   ║");
    println!("║  2. Sessions                   ║");
    println!("║  3. Memories                  ║");
    println!("║  4. Search                     ║");
    println!("║  5. Help                       ║");
    println!("║                                ║");
    println!("║  q: Quit                       ║");
    println!("╚══════════════════════════════════╝");

    if !projects.is_empty() {
        println!("\n📁 Projects: {} total\n", projects.len());
        for (i, p) in projects.iter().take(10).enumerate() {
            println!("  {}. {}", i + 1, p);
        }
    }

    println!("\nPress key...");
    stdout.flush()?;

    // Simple input loop
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode().ok();
    execute!(stdout, LeaveAlternateScreen).ok();
    println!("\n");

    Ok(())
}
