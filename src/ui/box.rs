use crossterm::{cursor, queue, style};

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// Rect
/// Represents a rectangle on a terminal grid of columns and rows.
/// A valid rectangle as at least one column and one row.
/// ////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Copy, Clone)]
pub struct Rect {
    /// X (begin, end)
    pub x: (u16, u16),
    /// Y (begin, end)
    pub y: (u16, u16),
}

impl Rect {
    /// Create from size.
    ///
    /// Assumes width and height are greater than zero.
    /// If unsure, call the "checked" function.
    pub fn from_size((x, y): (u16, u16), (width, height): (u16, u16)) -> Self {
        Self {
            x: (x, x + width - 1),
            y: (y, y + height - 1),
        }
    }

    /// Create checking that width and height are greater than zero.
    pub fn from_size_checked((x, y): (u16, u16), (width, height): (u16, u16)) -> Option<Self> {
        if width > 0 && height > 0 {
            Some(Self::from_size((x, y), (width, height)))
        } else {
            None
        }
    }

    /// Get width or number of columns
    pub fn width(&self) -> u16 {
        self.x.1 - self.x.0 + 1
    }

    /// Get height or number of rows
    pub fn height(&self) -> u16 {
        self.y.1 - self.y.0 + 1
    }

    /// Get width or number of columns
    pub fn cols(&self) -> u16 {
        self.width()
    }

    /// Get height or number of rows
    pub fn rows(&self) -> u16 {
        self.height()
    }

    /// Return an inner Rect.
    ///
    /// It is assumed current width and height are at least 3.
    /// If unsure, use the "checked" version.
    pub fn inner(&self) -> Self {
        Self {
            x: (self.x.0 + 1, self.x.1 - 1),
            y: (self.y.0 + 1, self.y.1 - 1),
        }
    }

    /// Return an inner Rect.
    ///
    /// Checks that the inner rectangle will be valid.
    pub fn inner_checked(&self) -> Option<Self> {
        if !self.valid() || self.width() < 3 || self.height() < 3 {
            None
        } else {
            Some(self.inner())
        }
    }

    /// Return an outer Rect.
    ///
    /// It is assumed current x0 and y0 are not at origin (0,0) but offset by at least 1.
    /// If unsure, use the "checked" version.
    pub fn outer(&self) -> Self {
        Self {
            x: (self.x.0 - 1, self.x.1 + 1),
            y: (self.y.0 - 1, self.y.1 + 1),
        }
    }

    /// Return an outer Rect.
    ///
    /// Checks that the inner rectangle will be valid.
    pub fn outer_checked(&self) -> Option<Self> {
        if !self.valid() || self.x.0 == 0 || self.y.0 == 0 {
            None
        } else {
            Some(self.outer())
        }
    }

    /// Return itself if Rect is still valid, otherwise consume itself and return None.
    pub fn checked(self) -> Option<Self> {
        if self.valid() {
            Some(self)
        } else {
            None
        }
    }

    /// Check if Self is a valid Rect.
    pub fn valid(&self) -> bool {
        self.x.0 < self.x.1 && self.y.0 < self.y.1
    }
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// Box
/// Box with borders, for drawing it in a terminal grid.
/// ////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct Box<'a> {
    pub rect: Rect,
    pub borders: &'a BorderChars,
}

impl<'a> Box<'a> {
    pub fn draw<W>(&self, stdout: &mut W)
    where
        W: std::io::Write,
    {
        // Only draw if we have inner area, (rect size is >= 3)
        let inner_rect = self.rect.inner_checked();
        if inner_rect.is_none() {
            return;
        }
        let inner_rect = inner_rect.unwrap();

        let horizontal = self
            .borders
            .horizontal
            .to_string()
            .repeat(inner_rect.width() as usize);

        // Top border
        queue!(
            stdout,
            cursor::MoveTo(self.rect.x.0, self.rect.y.0),
            style::Print(self.borders.upper_left),
            style::Print(&horizontal),
            style::Print(self.borders.upper_right),
        )
        .unwrap();

        // Bottom border
        queue!(
            stdout,
            cursor::MoveTo(self.rect.x.0, self.rect.y.1),
            style::Print(self.borders.lower_left),
            style::Print(&horizontal),
            style::Print(self.borders.lower_right),
        )
        .unwrap();

        // Left/Right borders
        for y in inner_rect.y.0..=inner_rect.y.1 {
            queue!(
                stdout,
                cursor::MoveTo(self.rect.x.0, y),
                style::Print(self.borders.vertical),
                cursor::MoveRight(inner_rect.cols()),
                style::Print(self.borders.vertical)
            )
            .unwrap();
        }
    }
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// Border Chars
/// ////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct BorderChars {
    pub upper_left: char,
    pub upper_right: char,
    pub lower_left: char,
    pub lower_right: char,
    pub horizontal: char,
    pub horizontal_up: char,
    pub horizontal_down: char,
    pub vertical: char,
    pub vertical_left: char,
    pub vertical_right: char,
}

impl BorderChars {
    pub fn simple() -> &'static Self {
        static SIMPLE: BorderChars = BorderChars {
            upper_left: '┌',
            upper_right: '┐',
            lower_left: '└',
            lower_right: '┘',
            horizontal: '─',
            horizontal_up: '┴',
            horizontal_down: '┬',
            vertical: '│',
            vertical_left: '┤',
            vertical_right: '├',
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
            horizontal_up: '╩',
            horizontal_down: '╦',
            vertical: '║',
            vertical_left: '╣',
            vertical_right: '╠',
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
            horizontal_up: '+',
            horizontal_down: '+',
            vertical: '|',
            vertical_left: '<',
            vertical_right: '>',
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
            horizontal_up: '┴',
            horizontal_down: '┬',
            vertical: '|',
            vertical_left: '┤',
            vertical_right: '├',
        };
        &SIMPLE_DASHED
    }
}
