use crate::commands::ui::util::event::{Event, Events};
use crate::config::CliConfig;
use crate::handler::get_remote_restapi_handler;
use crate::util;
use gerlib::changes::{AdditionalOpt, ChangeEndpoints, ChangeInfo, QueryParams, QueryStr};
use termion::event::Key;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Alignment, Direction, Rect};
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Row, Table, TableState};
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

struct StatefulTable {
    count: usize,
    state: TableState,
}

impl StatefulTable {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            state: TableState::default(),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < self.count - 1 {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
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

    let mut window_state = WindowState::new(4);
    let mut table_states = vec![
        StatefulTable::new(change_vec[0].len()),
        StatefulTable::new(change_vec[1].len()),
        StatefulTable::new(change_vec[2].len()),
    ];

    loop {
        terminal
            .draw(|frame| draw_dashboard(frame, &window_state, &mut table_states, &change_vec))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => break,
                Key::Char('J') => window_state.next(),
                Key::Char('K') => window_state.previous(),
                Key::Char('j') => {
                    if window_state.index < 3 {
                        table_states[window_state.index()].next()
                    }
                }
                Key::Char('k') => {
                    if window_state.index < 3 {
                        table_states[window_state.index()].previous()
                    }
                }
                _ => {}
            },
            Event::Resize => continue,
            Event::Tick => { /* Not transmitted at the moment */ }
        }
    }

    Ok(())
}

fn draw_dashboard<B>(
    frame: &mut Frame<B>, window_state: &WindowState, table_states: &mut Vec<StatefulTable>,
    change_vec: &Vec<Vec<ChangeInfo>>,
) where
    B: Backend,
{
    let windows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(frame.size());

    let list_windows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(windows[0]);

    let titles = ["Outgoing Reviews", "Incoming Reviews", "Recently Closed"];

    for i in 0..3 {
        draw_change_list(
            titles[i],
            frame,
            list_windows[i],
            window_state.index() == i,
            &mut table_states[i],
            &change_vec[i],
        );
    }

    if let Some(change) = change_vec[0].get(0) {
        draw_change_show(
            format!("Change {}", change.number).as_str(),
            frame,
            windows[1],
            window_state.index() == 3,
            &change,
        );
    }
}

fn draw_change_show<B>(
    title: &str, frame: &mut Frame<B>, window: Rect, selected: bool, change: &ChangeInfo,
) where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::raw("Status: "),
            Span::styled(
                if change.work_in_progress {
                    "WIP".to_owned()
                } else {
                    change.status.to_string()
                },
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(Span::raw(format!(
            "Owner: {} <{}>",
            change.owner.name.as_ref().unwrap(),
            change.owner.email.as_ref().unwrap(),
        ))),
        Spans::from(Span::raw(format!(
            "Updated: {}",
            util::format_long_datetime(&change.updated.0)
        ))),
        Spans::from(Span::raw(format!("Project: {}", change.project))),
        Spans::from(Span::raw(format!("Branch: {}", change.branch))),
        Spans::from(Span::raw(format!(
            "Commit: {}",
            change.current_revision.as_ref().unwrap()
        ))),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::DIM | Modifier::BOLD | Modifier::ITALIC),
                )),
        )
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, window);
}

fn draw_change_list<B>(
    title: &str, frame: &mut Frame<B>, window: Rect, selected: bool, state: &mut StatefulTable,
    changes: &Vec<ChangeInfo>,
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
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::DIM | Modifier::BOLD | Modifier::ITALIC),
                )),
        )
        .header_gap(0)
        .header_style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM))
        .highlight_style(Style::default().fg(Color::Yellow))
        .widths(&[
            Constraint::Length(6),
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Percentage(100),
        ]);
    frame.render_stateful_widget(table, window, &mut state.state);
}
