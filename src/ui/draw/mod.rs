//! This module contains the functions to draw the UI widgets.

// Exported draw functions:
pub use table::draw_table;
pub use winbox::draw_winbox;

mod table;
mod winbox;

use crate::ui::layout::HorizontalAlignment;

/// Format the string according to the available printable space.
///
/// Example:
/// content: "abc",   width: 5, alignment: Left,   output: "abc  "
/// content: "abc",   width: 5, alignment: Center, output: " abc "
/// content: "abc",   width: 5, alignment: Right,  output: "  abc"
/// content: "abc",   width: 3, alignment: *,      output: "abc"
/// content: "abcde", width: 3, alignment: *,      output: "ab…"
fn format_strip_align(content: &String, width: usize, alignment: &HorizontalAlignment) -> String {
  if width == 0 {
    Default::default()
  } else if content.len() > width {
    // Cut the string and append a etc. symbol to the end
    let (content_split, _) = content.split_at(width - 1);
    let content = format!("{}…", content_split);
    format_width_align(&content, width - 1, alignment)
  } else {
    // Fill blank with space character
    format_width_align(content, width, alignment)
  }
}

/// Format content with minimum width and desired horizontal alignment.
///
/// Additional space (the character) is added to fill up the width.
fn format_width_align(content: &String, width: usize, alignment: &HorizontalAlignment) -> String {
  match alignment {
    HorizontalAlignment::Left => format!("{: <1$}", content, width),
    HorizontalAlignment::Center => format!("{: ^1$}", content, width),
    HorizontalAlignment::Right => format!("{: >1$}", content, width),
  }
}
