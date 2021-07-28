use crate::ui::ecs_tui::EcsTui;
use crossterm::{
  cursor, event, execute, style,
  terminal::{self, ClearType},
};
use std::io::{BufWriter, Write};
use crate::config::CliConfig;

pub fn main(config: &mut CliConfig) {
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

  EcsTui::new(config).main_loop(&mut stdout, config);

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
