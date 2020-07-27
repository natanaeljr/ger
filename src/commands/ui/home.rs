use crate::commands::ui::util::event::{Event, Events};
use crate::config::CliConfig;
use crate::handler::get_remote_restapi_handler;
use gerlib::changes::{AdditionalOpt, ChangeEndpoints, ChangeInfo, QueryParams, QueryStr};
use termion::event::Key;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::{Backend, TermionBackend};
use tui::layout::Rect;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, Row, Table};
use tui::{Frame, Terminal};

struct WindowState {
    count: usize,
    index: usize,
}

impl WindowState {
    pub fn new(count: usize) -> Self {
        assert!(count > 0);
        Self { count, index: 0 }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn next(&mut self) {
        if self.index < self.count - 1 {
            self.index += 1;
        }
    }
    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}

pub fn main(config: &mut CliConfig) -> Result<(), failure::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut window_state = WindowState::new(3);

    let mut rest = get_remote_restapi_handler(config, None)?;
    let query_param = QueryParams {
        search_queries: Some(vec![
            QueryStr::Raw("is:open owner:self".into()),
            QueryStr::Raw("is:open reviewer:self -owner:self".into()),
            QueryStr::Raw("is:closed (owner:self OR reviewer:self) limit:10".into()),
        ]),
        additional_opts: Some(vec![
            AdditionalOpt::DetailedAccounts,
            AdditionalOpt::CurrentRevision,
        ]),
        limit: None,
        start: None,
    };
    let change_vec: Vec<Vec<ChangeInfo>> = rest.query_changes(&query_param)?;

    loop {
        terminal.draw(|frame| draw_dashboard(frame, &window_state, &change_vec))?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Char('J') => {
                    window_state.next();
                }
                Key::Char('K') => {
                    window_state.previous();
                }
                _ => {}
            }
        };
    }

    Ok(())
}

fn draw_dashboard<B>(
    mut frame: Frame<B>, window_state: &WindowState, change_vec: &Vec<Vec<ChangeInfo>>,
) where
    B: Backend,
{
    let windows = Layout::default()
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(frame.size());

    outgoing_reviews(
        &mut frame,
        windows[0],
        window_state.index() == 0,
        &change_vec[0],
    );
    incoming_reviews(
        &mut frame,
        windows[1],
        window_state.index() == 1,
        &change_vec[1],
    );
    recently_closed(
        &mut frame,
        windows[2],
        window_state.index() == 2,
        &change_vec[2],
    );
}

fn outgoing_reviews<B>(
    frame: &mut Frame<B>, window: Rect, selected: bool, changes: &Vec<ChangeInfo>,
) where
    B: Backend,
{
    let header = ["number", "project", "status", "subject"];
    let list: Vec<Vec<String>> = changes
        .into_iter()
        .map(|change| {
            vec![
                change.number.to_string(),
                change.project.to_string(),
                if change.work_in_progress {
                    "WIP".to_owned()
                } else {
                    change.status.to_string()
                },
                change.subject.to_string(),
            ]
        })
        .collect();
    let rows = list.iter().map(|row| Row::Data(row.iter()));
    let table = Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if selected {
                    Style::new().fg(Color::Yellow).modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .title("Outgoing Reviews")
                .title_style(Style::new().modifier(Modifier::BOLD | Modifier::ITALIC)),
        )
        .header_gap(0)
        .header_style(Style::new().modifier(Modifier::DIM))
        .widths(&[
            Constraint::Length(6),
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Percentage(100),
        ]);
    frame.render_widget(table, window);
}

fn incoming_reviews<B>(
    frame: &mut Frame<B>, window: Rect, selected: bool, changes: &Vec<ChangeInfo>,
) where
    B: Backend,
{
    let header = ["number", "project", "status", "subject"];
    let list: Vec<Vec<String>> = changes
        .into_iter()
        .map(|change| {
            vec![
                change.number.to_string(),
                change.project.to_string(),
                change.status.to_string(),
                change.subject.to_string(),
            ]
        })
        .collect();
    let rows = list.iter().map(|row| Row::Data(row.iter()));
    let table = Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if selected {
                    Style::new().fg(Color::Yellow).modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .title("Incoming Reviews")
                .title_style(Style::new().modifier(Modifier::BOLD | Modifier::ITALIC)),
        )
        .header_gap(0)
        .header_style(Style::new().modifier(Modifier::DIM))
        .widths(&[
            Constraint::Length(6),
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Percentage(100),
        ]);
    frame.render_widget(table, window);
}

fn recently_closed<B>(frame: &mut Frame<B>, window: Rect, selected: bool, changes: &Vec<ChangeInfo>)
where
    B: Backend,
{
    let header = ["number", "project", "status", "subject"];
    let list: Vec<Vec<String>> = changes
        .into_iter()
        .map(|change| {
            vec![
                change.number.to_string(),
                change.project.to_string(),
                change.status.to_string(),
                change.subject.to_string(),
            ]
        })
        .collect();
    let rows = list.iter().map(|row| Row::Data(row.iter()));
    let table = Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if selected {
                    Style::new().fg(Color::Yellow).modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .title("Recently Closed")
                .title_style(Style::new().modifier(Modifier::BOLD | Modifier::ITALIC)),
        )
        .header_gap(0)
        .header_style(Style::new().modifier(Modifier::DIM))
        .widths(&[
            Constraint::Length(6),
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Percentage(100),
        ]);
    frame.render_widget(table, window);
}
