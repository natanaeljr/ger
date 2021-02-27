use super::r#box::{BorderChars, Box, Rect};
use crossterm::style::{Attribute, Color, ContentStyle};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{self, ClearType},
};

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
    loop {
        // TODO: Update tick

        // Rendering
        queue!(stdout, style::ResetColor, terminal::Clear(ClearType::All)).unwrap();
        draw(stdout);
        stdout.flush().unwrap();

        // Event handling
        match event::read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        // TODO: Swap buffers (BufWriter)
    }
}

fn draw<W>(stdout: &mut W)
where
    W: std::io::Write,
{
    let term_size = terminal::size().unwrap();
    let window = Box {
        area: Rect {
            x: 0,
            y: 0,
            width: term_size.0,
            height: term_size.1,
        },
        borders: BorderChars::simple_dashed(),
    };
    window.draw(stdout);

    let inner = window.inner_area();

    let data = [
        [
            "8f524ac",
            "104508",
            "Auto QA",
            "11:24 AM",
            "packetsubsystem",
            "develop",
            "",
            "NEW",
            "Remove conditional that verifies if info is filled",
        ],
        [
            "8f524ac",
            "104508",
            "Auto QA",
            "11:24 AM",
            "packetsubsystem",
            "develop",
            "",
            "NEW",
            "Remove conditional that verifies if info is filled",
        ],
        [
            "8f524ac",
            "104508",
            "Auto QA",
            "11:24 AM",
            "packetsubsystem",
            "develop",
            "",
            "NEW",
            "Remove conditional that verifies if info is filled",
        ],
        [
            "8f524ac",
            "104508",
            "Auto QA",
            "11:24 AM",
            "packetsubsystem",
            "develop",
            "",
            "NEW",
            "Remove conditional that verifies if info is filled",
        ],
        [
            "8f524ac",
            "104508",
            "Auto QA",
            "11:24 AM",
            "packetsubsystem",
            "develop",
            "",
            "NEW",
            "Remove conditional that verifies if info is filled",
        ],
    ];

    let mut columns: Vec<(&str, u16, ContentStyle)> = Vec::new();
    columns.push(("commit", 8, ContentStyle::new().attribute(Attribute::Bold)));
    columns.push((
        "number",
        8,
        ContentStyle::new()
            .foreground(Color::DarkYellow)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "owner",
        16,
        ContentStyle::new()
            .foreground(Color::DarkGrey)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "time",
        10,
        ContentStyle::new()
            .foreground(Color::Magenta)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "project",
        30,
        ContentStyle::new()
            .foreground(Color::Cyan)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "branch",
        20,
        ContentStyle::new()
            .foreground(Color::DarkCyan)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "topic",
        20,
        ContentStyle::new()
            .foreground(Color::DarkCyan)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "status",
        10,
        ContentStyle::new()
            .foreground(Color::Green)
            .attribute(Attribute::Bold),
    ));
    columns.push((
        "subject",
        80,
        ContentStyle::new().attribute(Attribute::Bold),
    ));

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
        for row in 0..std::cmp::min(data.len(), (inner.height - 1) as usize) {
            let value = data[row][column]
                .split_at(std::cmp::min(column_len as usize, data[row][column].len()))
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
