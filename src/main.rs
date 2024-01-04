use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use std::io;

fn main() -> io::Result<()> {
    // Set up the terminal
    execute!(
        io::stdout(),
        terminal::SetTitle("Rusty Navigatior - CLI File Explorer"),
        terminal::SetSize(80, 24),
        terminal::Clear(ClearType::All)
    )?;

    println!("Welcome to Rusty Navigator - A Rust based CLI File Explorer");
    println!("Press 'q' to quit.");

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(event) = event::read()? {
                handle_key_event(event)?;
            }
        }
    }
}

fn handle_key_event(event: KeyEvent) -> io::Result<()> {
    match event.code {
        KeyCode::Char('q') => {
            // Quit the program
            execute!(io::stdout(), terminal::Clear(ClearType::All))?;
            std::process::exit(0);
        }

        _ => {}
    }
    Ok(())
}
