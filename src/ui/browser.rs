use crate::ui::ecs_tui::EcsTui;
use crossterm::{
    cursor, event, execute, style,
    terminal::{self, ClearType},
};
use std::io::{BufWriter, Write};

pub fn main() {
    let mut stdout = BufWriter::new(std::io::stdout()); // BufWriter decreases flickering

    terminal::enable_raw_mode().unwrap();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::SetTitle("Ger UI"),
        terminal::Clear(ClearType::All),
        event::EnableMouseCapture,
        cursor::Hide
    )
    .unwrap();

    EcsTui::new().main_loop(&mut stdout);

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        style::ResetColor,
        cursor::Show,
        event::DisableMouseCapture,
        terminal::SetTitle(""),
        terminal::LeaveAlternateScreen,
    )
    .unwrap();
    stdout.flush().unwrap();
    terminal::disable_raw_mode().unwrap();
}
