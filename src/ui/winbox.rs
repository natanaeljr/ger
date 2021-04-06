use crossterm::style::ContentStyle;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// UiBox
/// Box is a rect with borders, for drawing it to a terminal grid.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct WinBox {
    pub style: ContentStyle,
    pub borders: BorderChars,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Border Chars
////////////////////////////////////////////////////////////////////////////////////////////////////
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
