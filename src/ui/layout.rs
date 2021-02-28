#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
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
