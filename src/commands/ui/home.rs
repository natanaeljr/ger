use crate::commands::ui::util::event::{Event, Events};
use termion::event::Key;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::style::{Modifier, Style};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Row, Table},
    Frame, Terminal,
};

pub fn main() -> Result<(), failure::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    loop {
        terminal.draw(|frame| draw(frame))?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            }
        };
    }

    Ok(())
}

fn draw<B>(mut frame: Frame<B>)
where
    B: Backend,
{
    let windows = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(frame.size());
    let header = ["number", "project", "subject"];
    let data = [
        [
            "96895",
            "dmos-hal-filters-ll-bcm",
            "[US92241] Implement ActionMplsPortIgnoreVlanAndStgCheck",
        ],
        [
            "98677",
            "dmos-hal-switch-vpn-ll-bcm",
            "[US93084] Also remove encap when deleting all VPNs",
        ],
    ];
    let rows = data.iter().map(|value| Row::Data(value.iter()));
    let table = Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Outgoing Reviews")
                .title_style(Style::new().modifier(Modifier::BOLD | Modifier::ITALIC)),
        )
        .header_gap(0)
        .header_style(Style::new().modifier(Modifier::DIM))
        .widths(&[
            Constraint::Length(6),
            Constraint::Length(30),
            Constraint::Percentage(100),
        ]);
    frame.render_widget(table, windows[0]);
}
