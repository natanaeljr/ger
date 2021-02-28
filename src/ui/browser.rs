use super::r#box::{BorderChars, Box, Rect};
use crate::ui::scroll::{RangeTotal, ScrollBar, ScrollBarChars};
use crossterm::event::{KeyModifiers, MouseEventKind};
use crossterm::style::{Attribute, Color, ContentStyle, Styler};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{self, ClearType},
};

struct State<'a> {
    changelist: ChangeList<'a>,
}

pub fn main() {
    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode().unwrap();
    execute!(
        stdout,
        terminal::SetTitle("Ger UI"),
        terminal::EnterAlternateScreen,
        terminal::Clear(ClearType::All),
        event::EnableMouseCapture,
        cursor::Hide
    )
    .unwrap();

    main_loop(&mut stdout);

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        style::ResetColor,
        cursor::Show,
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen,
        terminal::SetTitle(""),
    )
    .unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn main_loop<W>(stdout: &mut W)
where
    W: std::io::Write,
{
    let mut state = init_state();

    loop {
        // TODO: Update tick

        // Rendering
        queue!(stdout, style::ResetColor, terminal::Clear(ClearType::All)).unwrap();
        draw(stdout, &state);
        stdout.flush().unwrap();

        // Event handling
        let mut quit = false;
        loop {
            match event::read().unwrap() {
                Event::Key(key) => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        match key.code {
                            KeyCode::Char('e') => {
                                if state.changelist.scroll(1) {
                                    break;
                                }
                            }
                            KeyCode::Char('y') => {
                                if state.changelist.scroll(-1) {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    if key.modifiers == KeyModifiers::empty() {
                        match key.code {
                            KeyCode::Char('q') => {
                                quit = true;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollDown => {
                        if state.changelist.scroll(1) {
                            break;
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if state.changelist.scroll(-1) {
                            break;
                        }
                    }
                    _ => {}
                },
                Event::Resize(cols, rows) => {
                    if cols >= 3 && rows >= 3 {
                        state.changelist.resize(cols, rows);
                        break;
                    }
                }
            }
        }
        if quit {
            break;
        }
    }
}

fn init_state<'a>() -> State<'a> {
    let (term_width, term_height) = terminal::size().unwrap();
    let changelist = ChangeList::new(Box {
        area: Rect {
            x: 0,
            y: 0,
            width: term_width,
            height: term_height,
        },
        borders: BorderChars::simple(),
    });
    State { changelist }
}

fn draw<W>(stdout: &mut W, state: &State)
where
    W: std::io::Write,
{
    let (term_width, term_height) = terminal::size().unwrap();
    if term_width < 3 || term_height < 3 {
        return;
    }
    let inner_area = state.changelist.box_.inner_area();

    // CHANGELIST
    let columns = get_columns();
    state.changelist.draw(stdout, &columns);

    // SCROLLBAR
    let scrollbar = ScrollBar {
        x: inner_area.x + inner_area.width - 1,
        y: inner_area.y,
        height: inner_area.height - 1,
        symbols: &ScrollBarChars::modern(),
    };
    scrollbar.draw(
        stdout,
        RangeTotal {
            begin: state.changelist.scrolled_rows,
            end: state.changelist.scrolled_rows + (inner_area.height - 1) as usize,
            total: DATA.len(),
        },
    );

    // HEADERS/FOOTERS
    let box_name = "change list";
    if state.changelist.box_.area.width > (box_name.len() as u16 + /*padding*/4) {
        queue!(
            stdout,
            cursor::MoveTo(
                state.changelist.box_.area.x + 2,
                state.changelist.box_.area.y
            ),
            style::Print(state.changelist.box_.borders.vertical_left),
            style::PrintStyledContent(style::style(box_name).with(Color::White).bold()),
            style::Print(state.changelist.box_.borders.vertical_right),
        )
        .unwrap();
    }

    let scrolled_end =
        state.changelist.scrolled_rows + (state.changelist.box_.inner_area().height - 1) as usize;
    let scrolled_end = std::cmp::min(scrolled_end, DATA.len());
    let visible_count = scrolled_end - state.changelist.scrolled_rows;
    let list_counts = format!("{}/{}", visible_count, DATA.len(),);
    if state.changelist.box_.area.width > (list_counts.len() as u16 + /*padding*/4) {
        let begin_x = state.changelist.box_.area.width - list_counts.len() as u16 - /*padding*/4;
        queue!(
            stdout,
            cursor::MoveTo(begin_x, state.changelist.box_.area.y,),
            style::Print(state.changelist.box_.borders.vertical_left),
            style::PrintStyledContent(style::style(list_counts).with(Color::White).bold()),
            style::Print(state.changelist.box_.borders.vertical_right),
        )
        .unwrap();
    }

    let scroll_range = format!("{}~{}", state.changelist.scrolled_rows + 1, scrolled_end);
    if state.changelist.box_.area.width > (scroll_range.len() as u16 + /*padding*/4)
        && (state.changelist.box_.inner_area().height as usize - 1) < DATA.len()
    {
        let begin_x = state.changelist.box_.area.width - scroll_range.len() as u16 - /*padding*/4;
        queue!(
            stdout,
            cursor::MoveTo(
                begin_x,
                state.changelist.box_.area.y + state.changelist.box_.area.height
            ),
            style::Print(state.changelist.box_.borders.vertical_left),
            style::PrintStyledContent(style::style(scroll_range).with(Color::White).bold()),
            style::Print(state.changelist.box_.borders.vertical_right),
        )
        .unwrap();
    }
}

struct ChangeList<'a> {
    box_: Box<'a>,
    scrolled_rows: usize,
    // TODO: implement show_header: bool,
}

impl<'a> ChangeList<'a> {
    pub fn new(box_: Box<'a>) -> Self {
        Self {
            box_,
            scrolled_rows: 0,
        }
    }

    pub fn scroll(&mut self, scroll_rows: i32) -> bool {
        let inner = self.box_.inner_area();
        let max_scroll = {
            // Bad math:
            let max_height = (inner.height - /*header*/1) as i32;
            let max_data_scroll = (DATA.len() - 1/*cause scroll starts on zero*/) as i32;
            let rows_after_scroll = (DATA.len() - self.scrolled_rows) as i32;
            let visible_rows = std::cmp::min(max_height, rows_after_scroll);
            if visible_rows == rows_after_scroll {
                self.scrolled_rows as i32
            } else {
                max_data_scroll
            }
        };
        let new_scroll = {
            let new_scroll = self.scrolled_rows as i32 + scroll_rows;
            if new_scroll < 0 {
                0
            } else if new_scroll > max_scroll {
                max_scroll
            } else {
                new_scroll
            }
        };
        let updated = self.scrolled_rows != new_scroll as usize;
        self.scrolled_rows = new_scroll as usize;
        return updated;
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let scroll_diff = (self.box_.area.height as i32) - (rows as i32);
        let rows_after_scroll = (DATA.len() - self.scrolled_rows) as i32;
        if (rows_after_scroll < (rows - 3) as i32) && scroll_diff.is_negative() {
            self.scroll(scroll_diff);
        }
        self.box_.area.width = cols;
        self.box_.area.height = rows;
    }

    pub fn draw<W>(&self, stdout: &mut W, columns: &Vec<(&str, u16, ContentStyle)>)
    where
        W: std::io::Write,
    {
        self.box_.draw(stdout);

        let inner = self.box_.inner_area();

        let mut walked_len = 0;
        for (column, (column_name, column_len, column_style)) in columns.iter().enumerate() {
            let remaining_len = {
                let value = (inner.width - walked_len) as i32;
                if value.is_positive() {
                    value as u16
                } else {
                    0 as u16
                }
            };

            // HEADER
            let column_len = std::cmp::min(column_len.clone(), remaining_len);
            let column_name = column_name
                .split_at(std::cmp::min(column_len as usize, column_name.len()))
                .0;
            queue!(
                stdout,
                cursor::MoveTo(inner.x + walked_len, inner.y),
                style::PrintStyledContent(style::StyledContent::new(*column_style, column_name))
            )
            .unwrap();

            // DATA
            let offset_row = self.scrolled_rows;
            for row in 0..std::cmp::min(DATA.len() - offset_row, (inner.height - 1) as usize) {
                let value = DATA[row + offset_row][column]
                    .split_at(std::cmp::min(
                        column_len as usize,
                        DATA[row + offset_row][column].len(),
                    ))
                    .0;
                queue!(
                    stdout,
                    cursor::MoveTo(inner.x + walked_len, inner.y + row as u16 + 1),
                    style::Print(value)
                )
                .unwrap();
            }

            walked_len += column_len;
        }
    }
}

fn get_columns() -> Vec<(&'static str, u16, ContentStyle)> {
    let columns = vec![
        ("commit", 8, ContentStyle::new().attribute(Attribute::Bold)),
        (
            "number",
            8,
            ContentStyle::new()
                .foreground(Color::DarkYellow)
                .attribute(Attribute::Bold),
        ),
        (
            "owner",
            17,
            ContentStyle::new()
                .foreground(Color::DarkGrey)
                .attribute(Attribute::Bold),
        ),
        (
            "time",
            10,
            ContentStyle::new()
                .foreground(Color::Magenta)
                .attribute(Attribute::Bold),
        ),
        (
            "project",
            30,
            ContentStyle::new()
                .foreground(Color::Cyan)
                .attribute(Attribute::Bold),
        ),
        (
            "branch",
            20,
            ContentStyle::new()
                .foreground(Color::DarkCyan)
                .attribute(Attribute::Bold),
        ),
        (
            "topic",
            20,
            ContentStyle::new()
                .foreground(Color::DarkCyan)
                .attribute(Attribute::Bold),
        ),
        (
            "status",
            10,
            ContentStyle::new()
                .foreground(Color::Green)
                .attribute(Attribute::Bold),
        ),
        (
            "subject",
            80,
            ContentStyle::new().attribute(Attribute::Bold),
        ),
    ];
    columns
}

static DATA: [[&str; 9]; 45] = [
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "issue648",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
    // REPEAT
    [
        "8f524ac",
        "104508",
        "Auto QA",
        "11:24 AM",
        "packets-system",
        "develop",
        "",
        "NEW",
        "Remove conditional that verifies if info is filled",
    ],
    [
        "18d3290",
        "104525",
        "Joao Begin",
        "07:16 PM",
        "feature-center",
        "future",
        "diag",
        "NEW",
        "Add diagnostics feature to some platforms",
    ],
    [
        "46a003e",
        "104455",
        "Jonas Merker",
        "06:33 PM",
        "wifi-wire",
        "develop",
        "bug438422",
        "MERGED",
        "[Bug 438422] check hardware for connected",
    ],
    [
        "e810b1c",
        "104451",
        "Thomas Silvester",
        "07:19 PM",
        "ip-protocols",
        "develop",
        "",
        "ABANDONED",
        "Use local configuration for application protocols",
    ],
    [
        "f5d606d",
        "104539",
        "Eduard Smith",
        "07:36 PM",
        "hw-scripts",
        "master",
        "",
        "DRAFT",
        "Fix FAN instability",
    ],
];
