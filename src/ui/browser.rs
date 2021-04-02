use super::r#box::{BorderChars, Box, Rect};
use crate::ui::ecs_tui::EcsTui;
use crate::ui::scroll::{RangeTotal, ScrollBar, ScrollBarChars};
use crossterm::event::{KeyModifiers, MouseButton, MouseEventKind};
use crossterm::style::{Attribute, Color, ContentStyle, StyledContent, Styler};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{self, ClearType},
};
use std::io::{BufWriter, Write};

struct State<'a> {
    changelist: ChangeList<'a>,
}

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

    // main_loop(&mut stdout);
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

fn main_loop<W>(stdout: &mut W)
where
    W: std::io::Write,
{
    let mut state = init_state();
    loop {
        // Rendering
        queue!(stdout, style::ResetColor, terminal::Clear(ClearType::All)).unwrap();
        draw(stdout, &state);
        stdout.flush().unwrap();

        // Event handling
        let mut quit = false;
        event_loop(&mut state, &mut quit);
        if quit {
            break;
        }
    }
}

fn event_loop(state: &mut State, quit: &mut bool) {
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
                        KeyCode::Char('d') => {
                            let inner = state.changelist.r#box.rect.inner();
                            let middle = ((inner.height() as f32 - 1.0) / 2.0).ceil() as i32;
                            // TODO: Fix this, it's not quite the same VI behaviour
                            if state.changelist.select_offset(middle) {
                                break;
                            }
                        }
                        KeyCode::Char('u') => {
                            let inner = state.changelist.r#box.rect.inner();
                            let middle = ((inner.height() as f32 - 1.0) / 2.0).ceil() as i32;
                            // TODO: Fix this, it's not quite the same VI behaviour
                            if state.changelist.select_offset(-middle) {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if key.modifiers == KeyModifiers::SHIFT {
                    match key.code {
                        KeyCode::Char('G') => {
                            if state.changelist.select_line(DATA.len() as i32 - 1) {
                                break;
                            }
                        }
                        KeyCode::Char('M') => {
                            let inner = state.changelist.r#box.rect.inner();
                            let middle = (inner.height() as i32 - 1) / 2;
                            if state.changelist.select_row(middle) {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if key.modifiers == KeyModifiers::empty() {
                    match key.code {
                        KeyCode::Char('q') => {
                            *quit = true;
                            break;
                        }
                        KeyCode::Char('g') => {
                            if state.changelist.select_line(0) {
                                break;
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if state.changelist.select_offset(1) {
                                break;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if state.changelist.select_offset(-1) {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    if state.changelist.scroll(3) {
                        break;
                    }
                }
                MouseEventKind::ScrollUp => {
                    if state.changelist.scroll(-3) {
                        break;
                    }
                }
                MouseEventKind::Drag(button) => match button {
                    MouseButton::Left => {
                        let inner = state.changelist.r#box.rect.inner();
                        // Scroll Bar
                        if inner.height() > 1 {
                            if state.changelist.bar_clicking
                                && mouse.row > inner.y.0
                                && mouse.row < inner.y.1
                            {
                                state.changelist.barscroll(mouse.row);
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                MouseEventKind::Down(button) => match button {
                    MouseButton::Left => {
                        let inner = state.changelist.r#box.rect.inner();
                        // Scroll Arrows
                        if mouse.row == inner.y.0 && mouse.column == inner.x.1 {
                            state.changelist.up_arrow_click = true;
                            if state.changelist.scroll(-1) {
                                break;
                            }
                        } else if mouse.row == inner.y.1 && mouse.column == inner.x.1 {
                            state.changelist.down_arrow_click = true;
                            if state.changelist.scroll(1) {
                                break;
                            }
                        }
                        // Scroll Bar
                        if inner.height() > 1 {
                            if mouse.row > inner.y.0
                                && mouse.row < inner.y.1
                                && mouse.column == inner.x.1
                            {
                                state.changelist.bar_clicking = true;
                                state.changelist.barscroll(mouse.row);
                                break;
                            }
                        }
                        // List Entries
                        if mouse.row > inner.y.0
                            && mouse.row <= inner.y.1
                            && mouse.column >= inner.x.0
                            && mouse.column <= inner.x.1
                        {
                            if state
                                .changelist
                                .select_row(mouse.row as i32 - inner.y.0 as i32)
                            {
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                MouseEventKind::Up(button) => match button {
                    MouseButton::Left => {
                        if state.changelist.bar_clicking {
                            state.changelist.bar_clicking = false;
                            break;
                        }
                        if state.changelist.up_arrow_click {
                            state.changelist.up_arrow_click = false;
                            break;
                        }
                        if state.changelist.down_arrow_click {
                            state.changelist.down_arrow_click = false;
                            break;
                        }
                    }
                    _ => {}
                },
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
}

fn init_state<'a>() -> State<'a> {
    let (term_width, term_height) = terminal::size().unwrap();
    let changelist = ChangeList::new(Box {
        rect: Rect::from_size((0, 0), (term_width, term_height)),
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
    let inner_area = state.changelist.r#box.rect.inner();

    // CHANGELIST
    let columns = get_columns();
    state.changelist.draw(stdout, &columns);

    // SCROLLBAR
    let scrollbar = ScrollBar {
        x: inner_area.x.0 + inner_area.width() - 1,
        y: inner_area.y.0,
        height: inner_area.height() - 1,
        symbols: ScrollBarChars::modern(),
    };
    scrollbar.draw(
        stdout,
        RangeTotal {
            begin: state.changelist.scrolled_rows,
            end: state.changelist.scrolled_rows + (inner_area.height() - 1) as usize,
            total: DATA.len(),
        },
        state.changelist.bar_clicking,
        state.changelist.up_arrow_click,
        state.changelist.down_arrow_click,
    );

    // HEADERS/FOOTERS
    let box_name = "change list";
    if state.changelist.r#box.rect.width() > (box_name.len() as u16 + /*padding*/4) {
        queue!(
            stdout,
            cursor::MoveTo(
                state.changelist.r#box.rect.x.0 + 2,
                state.changelist.r#box.rect.y.0
            ),
            style::Print(state.changelist.r#box.borders.vertical_left),
            style::PrintStyledContent(style::style(box_name).with(Color::White).bold()),
            style::Print(state.changelist.r#box.borders.vertical_right),
        )
        .unwrap();
    }

    let scrolled_end = state.changelist.scrolled_rows
        + (state.changelist.r#box.rect.inner().height() - 1) as usize;
    let scrolled_end = std::cmp::min(scrolled_end, DATA.len());
    let visible_count = scrolled_end - state.changelist.scrolled_rows;
    let list_counts = format!("{}/{}", visible_count, DATA.len(),);
    if state.changelist.r#box.rect.width() > (list_counts.len() as u16 + /*padding*/4) {
        let begin_x = state.changelist.r#box.rect.width() - list_counts.len() as u16 - /*padding*/4;
        queue!(
            stdout,
            cursor::MoveTo(begin_x, state.changelist.r#box.rect.y.0),
            style::Print(state.changelist.r#box.borders.vertical_left),
            style::PrintStyledContent(style::style(list_counts).with(Color::White).bold()),
            style::Print(state.changelist.r#box.borders.vertical_right),
        )
        .unwrap();
    }

    let scroll_range = format!("{}~{}", state.changelist.scrolled_rows + 1, scrolled_end);
    if state.changelist.r#box.rect.width() > (scroll_range.len() as u16 + /*padding*/4)
        && (state.changelist.r#box.rect.inner().height() as usize - 1) < DATA.len()
    {
        let begin_x =
            state.changelist.r#box.rect.width() - scroll_range.len() as u16 - /*padding*/4;
        queue!(
            stdout,
            cursor::MoveTo(begin_x, state.changelist.r#box.rect.y.1),
            style::Print(state.changelist.r#box.borders.vertical_left),
            style::PrintStyledContent(style::style(scroll_range).with(Color::White).bold()),
            style::Print(state.changelist.r#box.borders.vertical_right),
        )
        .unwrap();
    }
}

struct ChangeList<'a> {
    r#box: Box<'a>,
    up_arrow_click: bool,
    down_arrow_click: bool,
    bar_clicking: bool,
    scrolled_rows: usize,
    selected_row: usize,
    show_line_numbers: bool, // TODO: (hide/normal/relative)
                             // TODO: implement show_column_headers: bool,
}

impl<'a> ChangeList<'a> {
    pub fn new(box_: Box<'a>) -> Self {
        Self {
            r#box: box_,
            up_arrow_click: false,
            down_arrow_click: false,
            bar_clicking: false,
            scrolled_rows: 0,
            selected_row: 0,
            show_line_numbers: true,
        }
    }

    pub fn select_offset(&mut self, offset: i32) -> bool {
        let target = self.selected_row as i32 + offset;
        let target = std::cmp::max(target, 0);
        let target = std::cmp::min(target, DATA.len() as i32 - 1);
        let updated = target != self.selected_row as i32;
        if updated {
            self.selected_row = target as usize;
            self.scroll_on_select_update();
        }
        updated
    }

    pub fn select_line(&mut self, line: i32) -> bool {
        let line = std::cmp::max(line, 0);
        let line = std::cmp::min(line, DATA.len() as i32 - 1);
        let updated = line != self.selected_row as i32;
        if updated {
            self.selected_row = line as usize;
            self.scroll_on_select_update();
        }
        updated
    }

    pub fn select_row(&mut self, row: i32) -> bool {
        let row = self.scrolled_rows as i32 + row - /*header*/1;
        let row = std::cmp::max(row, 0);
        let row = std::cmp::min(row, DATA.len() as i32 - 1);
        let updated = row != self.selected_row as i32;
        if updated {
            self.selected_row = row as usize;
            self.scroll_on_select_update();
        }
        updated
    }

    fn scroll_on_select_update(&mut self) {
        let inner = self.r#box.rect.inner();
        let scrolled_visible_end = self.scrolled_rows as i32 + (inner.height() as i32 - 2);
        if self.selected_row < self.scrolled_rows {
            self.scroll_internal(self.selected_row as i32 - self.scrolled_rows as i32);
        } else if self.selected_row as i32 > scrolled_visible_end {
            self.scroll_internal(self.selected_row as i32 - scrolled_visible_end);
        }
    }

    fn select_on_scroll_or_resize_update(&mut self) {
        if self.selected_row < self.scrolled_rows {
            self.select_line(self.scrolled_rows as i32);
        }
        let inner = self.r#box.rect.inner();
        let scrolled_visible_end = self.scrolled_rows as i32 + (inner.height() as i32 - 2);
        if self.selected_row as i32 > scrolled_visible_end {
            self.select_line(scrolled_visible_end);
        }
    }

    pub fn scroll(&mut self, scroll_rows: i32) -> bool {
        let updated = self.scroll_internal(scroll_rows);
        self.select_on_scroll_or_resize_update();
        updated
    }

    pub fn scroll_internal(&mut self, scroll_rows: i32) -> bool {
        let inner = self.r#box.rect.inner();
        let scroll_rows = std::cmp::max(scroll_rows, -(inner.height() as i32 - 1));
        let scroll_rows = std::cmp::min(scroll_rows, inner.height() as i32 - 1);
        let max_scroll = {
            // Bad math:
            let max_height = (inner.height() - /*header*/1) as i32;
            let max_data_scroll = DATA.len() as i32;
            let rows_after_scroll = (DATA.len() - self.scrolled_rows) as i32;
            let visible_scrolled_rows = std::cmp::min(max_height, rows_after_scroll);
            max_data_scroll - visible_scrolled_rows
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
        updated
    }

    pub fn barscroll(&mut self, row: u16) -> bool {
        let inner_area = self.r#box.rect.inner();
        let y = inner_area.y.0;
        let height = inner_area.height() - 1;
        let range_shown = RangeTotal {
            begin: self.scrolled_rows,
            end: self.scrolled_rows + (inner_area.height() - 1) as usize,
            total: DATA.len(),
        };
        let bar_ypos = ScrollBar::bar_ypos(height, y, &range_shown);
        let positions = ScrollBar::positions(height, y, &range_shown);
        let mut final_pos = 0;
        for pos in positions.iter() {
            let old_diff = (final_pos as i32 - *pos as i32).abs();
            let new_diff = (row as i32 - *pos as i32).abs();
            if new_diff < old_diff {
                final_pos = row;
            }
        }
        if bar_ypos == final_pos {
            return false;
        }
        self.scrolled_rows = ScrollBar::scroll(height, y, &range_shown, final_pos) as usize;
        self.select_on_scroll_or_resize_update();
        true
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let scroll_diff = (self.r#box.rect.height() as i32) - (rows as i32);
        let rows_after_scroll = (DATA.len() - self.scrolled_rows) as i32;
        if (rows_after_scroll < (rows - 3) as i32) && scroll_diff.is_negative() {
            self.scroll(scroll_diff);
        }
        self.r#box.rect = Rect::from_size((self.r#box.rect.x.0, self.r#box.rect.y.0), (cols, rows));
        self.select_on_scroll_or_resize_update();
    }

    pub fn draw<W>(&self, stdout: &mut W, columns: &Vec<(&str, u16, ContentStyle)>)
    where
        W: std::io::Write,
    {
        // This whole function is really bad :( I'm sorry.
        self.r#box.draw(stdout);
        let (term_width, _term_height) = terminal::size().unwrap();

        let mut inner = self.r#box.rect.inner();
        if inner.height() as usize - 1 < DATA.len() && inner.width() > 1 {
            // consider the scrollbar
            inner = Rect::from_size((inner.x.0, inner.y.0), (inner.width() - 1, inner.height()));
        }

        let mut walked_len = 0;
        for (column, (column_name, column_len, column_style)) in columns.iter().enumerate() {
            if self.show_line_numbers && column == 0 {
                let digits_count = {
                    let digits_count = DATA.len().to_string().len();
                    if digits_count < 2 {
                        2 // always at least 2 digits
                    } else {
                        digits_count
                    }
                };
                walked_len += digits_count as u16 + 1;
            }

            let remaining_len = {
                let value = inner.width() as i32 - walked_len as i32;
                if value.is_positive() {
                    value as u16
                } else {
                    0 as u16
                }
            };

            // HEADER
            let column_len = std::cmp::min(column_len.clone(), remaining_len);
            let (column_name, rest) =
                column_name.split_at(std::cmp::min(column_len as usize, column_name.len()));
            let mut column_name = column_name.to_owned();
            if !rest.is_empty() && column_name.len() >= 1 {
                column_name = column_name.split_at(column_name.len() - 1).0.to_owned();
                column_name.push('…');
            }
            queue!(
                stdout,
                cursor::MoveTo(inner.x.0 + walked_len, inner.y.0),
                style::PrintStyledContent(style::StyledContent::new(*column_style, &column_name))
            )
            .unwrap();

            walked_len += column_len;
        }

        // DATA
        let offset_row = self.scrolled_rows;
        for row in 0..std::cmp::min(DATA.len() - offset_row, (inner.height() - 1) as usize) {
            let mut line = Vec::new();
            let mut walked_len = 0;
            queue!(
                line,
                cursor::MoveTo(inner.x.0, inner.y.0 + row as u16 + /*header*/1),
            )
            .unwrap();
            let style = if (row + offset_row) == self.selected_row {
                ContentStyle::new().attribute(Attribute::Underlined)
            } else {
                ContentStyle::new()
            };
            for (column, (_column_name, column_len, _column_style)) in columns.iter().enumerate() {
                if self.show_line_numbers && column == 0 {
                    let digits_count = {
                        let digits_count = DATA.len().to_string().len();
                        if digits_count < 2 {
                            2 // always at least 2 digits
                        } else {
                            digits_count
                        }
                    };
                    walked_len += digits_count as u16 + 1;
                    if inner.width() > digits_count as u16 {
                        queue!(
                            line,
                            style::Print(StyledContent::new(
                                style.foreground(Color::White).attribute(Attribute::Bold),
                                format!("{: >1$} ", row + offset_row + 1, digits_count)
                            )),
                        )
                        .unwrap();
                    }
                }
                let remaining_len = {
                    let value = inner.width() as i32 - walked_len as i32;
                    if value.is_positive() {
                        value as u16
                    } else {
                        0 as u16
                    }
                };
                let column_len = std::cmp::min(column_len.clone(), remaining_len);
                let (value, rest) = DATA[row + offset_row][column].split_at(std::cmp::min(
                    column_len as usize,
                    DATA[row + offset_row][column].len(),
                ));
                let mut value = value.to_owned();
                if !rest.is_empty() && value.len() >= 1 {
                    value = value.split_at(value.len() - 1).0.to_owned();
                    value.push('…');
                }
                let full_width = if column == (columns.len() - 1) {
                    let remaining_term_width =
                        term_width as i32 - walked_len as i32 - /*borders*/ 2;
                    std::cmp::max(column_len as i32, remaining_term_width) as u16
                } else {
                    column_len
                };
                let value = format!("{: <1$}", value, full_width as usize);
                queue!(line, style::Print(StyledContent::new(style, value))).unwrap();

                walked_len += column_len;
            }
            stdout.write_all(line.as_slice()).unwrap();
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

pub static DATA: [[&str; 9]; 45] = [
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
