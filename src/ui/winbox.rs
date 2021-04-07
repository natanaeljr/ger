use crate::ui::layout::{HorizontalAlignment, HorizontalMargin};
use crossterm::style::ContentStyle;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// WinBox is a rect with borders, for drawing it to a terminal grid.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct WinBox {
    pub style: ContentStyle,
    pub borders: BorderChars,
    pub top_hints: Vec<BoxHint>,    // order in the vector means priority
    pub bottom_hints: Vec<BoxHint>, // order in the vector means priority
}

/// Box hints can be used to add a title to the window box, or other status around the borders.
#[derive(Debug, Clone)]
pub struct BoxHint {
    pub content: String,
    pub style: ContentStyle,
    pub margin: HorizontalMargin,
    pub alignment: HorizontalAlignment,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Border Chars
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
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
