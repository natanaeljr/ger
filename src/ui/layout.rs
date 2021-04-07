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
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VerticalMargin {
    top: usize,
    bottom: usize,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct HorizontalMargin {
    pub left: usize,
    pub right: usize,
}
