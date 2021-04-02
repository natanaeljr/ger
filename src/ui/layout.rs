#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineNumberMode {
    Normal,
    Relative, // to the cursor
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Margin {
    top: u16,
    left: u16,
    right: u16,
    bottom: u16,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VerticalMargin {
    top: u16,
    bottom: u16,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct HorizontalMargin {
    left: u16,
    right: u16,
}
