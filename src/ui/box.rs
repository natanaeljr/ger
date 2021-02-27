use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{self, ClearType},
};

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct BorderChars {
    pub upper_left: char,
    pub upper_right: char,
    pub lower_left: char,
    pub lower_right: char,
    pub horizontal: char,
    pub vertical: char,
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
pub struct Box<'a> {
    pub area: Rect,
    pub borders: &'a BorderChars,
}

impl<'a> Box<'a> {
    pub fn inner_area(&self) -> Rect {
        Rect {
            x: self.area.x + 1,
            y: self.area.y + 1,
            width: self.area.width - 2,
            height: self.area.height - 2,
        }
    }
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
