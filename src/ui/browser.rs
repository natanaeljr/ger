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

    run_loop(&mut stdout);

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

fn run_loop<W>(stdout: &mut W)
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
        borders: BorderChars::simple(),
    };
    window.draw(stdout);
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[derive(Debug, Copy, Clone)]
struct BorderChars {
    upper_left: char,
    upper_right: char,
    lower_left: char,
    lower_right: char,
    horizontal: char,
    vertical: char,
}

impl BorderChars {
    pub fn simple() -> &'static Self {
        static SIMPLE: BorderChars = BorderChars {
            upper_left: '┌',
            upper_right: '┐',
            lower_left: '└',
            lower_right: '┘',
            horizontal: '─',
            vertical: '│',
        };
        &SIMPLE
    }
    pub fn double() -> &'static Self {
        static DOUBLE: BorderChars = BorderChars {
            upper_left: '╔',
            upper_right: '╗',
            lower_left: '╚',
            lower_right: '╝',
            horizontal: '═',
            vertical: '║',
        };
        &DOUBLE
    }
    pub fn dashed() -> &'static Self {
        static DASHED: BorderChars = BorderChars {
            upper_left: '+',
            upper_right: '+',
            lower_left: '+',
            lower_right: '+',
            horizontal: '-',
            vertical: '|',
        };
        &DASHED
    }
    pub fn simple_dashed() -> &'static Self {
        static SIMPLE_DASHED: BorderChars = BorderChars {
            upper_left: '┌',
            upper_right: '┐',
            lower_left: '└',
            lower_right: '┘',
            horizontal: '-',
            vertical: '|',
        };
        &SIMPLE_DASHED
    }
}

#[derive(Debug, Copy, Clone)]
struct Box<'a> {
    area: Rect,
    borders: &'a BorderChars,
}

impl<'a> Box<'a> {
    pub fn draw<W>(&self, stdout: &mut W)
    where
        W: std::io::Write,
    {
        // TODO: constrain creation of Box to width/height >= 2

        let inner_y = self.area.y + 1;
        let inner_width = self.area.width - 2;
        let inner_height = self.area.height - 2;
        let height_y = self.area.y + self.area.height - 1;
        let horizontal = self
            .borders
            .horizontal
            .to_string()
            .repeat(inner_width as usize);

        // top border
        queue!(
            stdout,
            cursor::MoveTo(self.area.x, self.area.y),
            style::Print(self.borders.upper_left),
            style::Print(&horizontal),
            style::Print(self.borders.upper_right),
        )
        .unwrap();

        // bottom border
        queue!(
            stdout,
            cursor::MoveTo(self.area.x, height_y),
            style::Print(self.borders.lower_left),
            style::Print(&horizontal),
            style::Print(self.borders.lower_right),
        )
        .unwrap();

        // left/right borders
        for y in inner_y..(inner_y + inner_height) {
            queue!(
                stdout,
                cursor::MoveTo(self.area.x, y),
                style::Print(self.borders.vertical),
                cursor::MoveRight(inner_width),
                style::Print(self.borders.vertical)
            )
            .unwrap();
        }
    }
}
