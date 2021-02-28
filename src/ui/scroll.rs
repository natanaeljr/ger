use crate::ui::list::List;
use crate::ui::r#box::Rect;
use crossterm::style::{Attribute, ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Scroll {
    pub top_row: u16,
}

pub struct CharStyle {
    pub char: char,
    pub style: ContentStyle,
}

pub struct ScrollBarChars {
    pub up: CharStyle,
    pub up_clicked: CharStyle,
    pub up_disabled: CharStyle,
    pub bar: CharStyle,
    pub bar_clicked: CharStyle,
    pub down: CharStyle,
    pub down_clicked: CharStyle,
    pub down_disabled: CharStyle,
    pub space: CharStyle,
    pub space_clicked: CharStyle,
}

impl ScrollBarChars {
    pub fn modern() -> Self {
        Self {
            up: CharStyle {
                char: '↑',
                style: ContentStyle::new().attribute(Attribute::Bold),
            },
            up_clicked: CharStyle {
                char: '↑',
                style: ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Reverse),
            },
            up_disabled: CharStyle {
                char: '↑',
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold),
            },
            bar: CharStyle {
                char: '█',
                style: ContentStyle::new(),
            },
            bar_clicked: CharStyle {
                char: '█',
                style: ContentStyle::new().attribute(Attribute::Bold),
            },
            down: CharStyle {
                char: '↓',
                style: ContentStyle::new().attribute(Attribute::Bold),
            },
            down_clicked: CharStyle {
                char: '↓',
                style: ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Reverse),
            },
            down_disabled: CharStyle {
                char: '↓',
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold),
            },
            space: CharStyle {
                char: ' ',
                style: ContentStyle::new(),
            },
            space_clicked: CharStyle {
                char: '·',
                style: ContentStyle::new().attribute(Attribute::Dim),
            },
        }
    }
    pub fn simple() -> Self {
        Self {
            up: CharStyle {
                char: '^',
                style: ContentStyle::new().attribute(Attribute::Bold),
            },
            up_clicked: CharStyle {
                char: '^',
                style: ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Reverse),
            },
            up_disabled: CharStyle {
                char: '^',
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold),
            },
            bar: CharStyle {
                char: '*',
                style: ContentStyle::new(),
            },
            bar_clicked: CharStyle {
                char: '*',
                style: ContentStyle::new().attribute(Attribute::Bold),
            },
            down: CharStyle {
                char: 'v',
                style: ContentStyle::new().attribute(Attribute::Bold),
            },
            down_clicked: CharStyle {
                char: 'v',
                style: ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Reverse),
            },
            down_disabled: CharStyle {
                char: 'v',
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold),
            },
            space: CharStyle {
                char: '|',
                style: ContentStyle::new(),
            },
            space_clicked: CharStyle {
                char: '|',
                style: ContentStyle::new().attribute(Attribute::Dim),
            },
        }
    }
}

pub struct ScrollBar<'a> {
    pub x: u16,
    pub y: u16,
    pub height: u16,
    pub symbols: &'a ScrollBarChars,
}

pub struct RangeTotal {
    pub begin: usize,
    pub end: usize,
    pub total: usize,
}

impl<'a> ScrollBar<'a> {
    pub fn draw<W>(&self, stdout: &mut W, range_shown: RangeTotal)
    where
        W: std::io::Write,
    {
        // verifications
        if self.height < 1 {
            return;
        }
        let visible_count = range_shown.end - range_shown.begin;
        if visible_count >= (range_shown.total) {
            return;
        }
        let max_begin = range_shown.total - (visible_count);

        struct Symbols<'a> {
            up: &'a CharStyle,
            down: &'a CharStyle,
            bar: &'a CharStyle,
            space: &'a CharStyle,
        }
        let symbols = Symbols {
            up: if range_shown.begin == 0 {
                &self.symbols.up_disabled
            } else {
                &self.symbols.up
            },
            down: if range_shown.begin == max_begin {
                &self.symbols.down_disabled
            } else {
                &self.symbols.down
            },
            bar: &self.symbols.bar,
            space: &self.symbols.space,
        };

        // SPACE
        for y in (self.y + 1)..(self.y + self.height) {
            queue!(
                stdout,
                cursor::MoveTo(self.x, y),
                style::PrintStyledContent(StyledContent::new(
                    symbols.space.style,
                    symbols.space.char
                )),
            )
            .unwrap();
        }
        // UP
        queue!(
            stdout,
            cursor::MoveTo(self.x, self.y),
            style::PrintStyledContent(StyledContent::new(symbols.up.style, symbols.up.char)),
        )
        .unwrap();
        // BAR
        if self.height > 1 {
            let bar_ypos_value = {
                // ratio: 0 ~ 1.0
                let ratio = range_shown.begin as f32 / max_begin as f32;
                (ratio * (self.height - /*arrows*/2) as f32) + (self.y + /*up arrow*/1) as f32
            };
            queue!(
                stdout,
                cursor::MoveTo(self.x, bar_ypos_value as u16),
                style::PrintStyledContent(StyledContent::new(symbols.bar.style, symbols.bar.char)),
            )
            .unwrap();
        }
        // DOWN
        queue!(
            stdout,
            cursor::MoveTo(self.x, self.y + self.height),
            style::PrintStyledContent(StyledContent::new(symbols.down.style, symbols.down.char)),
        )
        .unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn scroll_list(scroll: &mut Scroll, list: &List, rect: &Rect, offset: i32) -> bool {
    let max_scroll = {
        // Bad math:
        let max_height = (rect.height() - /*header*/1) as i32;
        let max_data_scroll = (list.data.len() - 1/*cause scroll starts on zero*/) as i32;
        let rows_after_scroll = (list.data.len() - scroll.top_row as usize) as i32;
        let visible_rows = std::cmp::min(max_height, rows_after_scroll);
        if visible_rows == rows_after_scroll {
            scroll.top_row as i32
        } else {
            max_data_scroll
        }
    };
    let new_scroll = {
        let new_scroll = scroll.top_row as i32 + offset;
        if new_scroll < 0 {
            0
        } else if new_scroll > max_scroll {
            max_scroll
        } else {
            new_scroll
        }
    };
    let updated = scroll.top_row != new_scroll as u16;
    scroll.top_row = new_scroll as u16;
    return updated;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ScrollBarModern<'a> {
    pub symbols: &'a ScrollBarChars,
}

pub fn draw_scrollbar<W>(
    stdout: &mut W, scrollbar: &ScrollBarModern, scroll: &Scroll, list: &List, rect: &Rect,
) where
    W: std::io::Write,
{
    let range_shown = RangeTotal {
        begin: scroll.top_row as usize,
        end: scroll.top_row as usize + (rect.height() - /*header*/1) as usize,
        total: list.data.len(),
    };

    // verifications
    if rect.height() < 2 {
        return;
    }
    let visible_count = range_shown.end - range_shown.begin;
    if visible_count >= (range_shown.total) {
        return;
    }
    let max_begin = range_shown.total - (visible_count);

    struct Symbols<'a> {
        up: &'a CharStyle,
        down: &'a CharStyle,
        bar: &'a CharStyle,
        space: &'a CharStyle,
    }
    let symbols = Symbols {
        up: if range_shown.begin == 0 {
            &scrollbar.symbols.up_disabled
        } else {
            &scrollbar.symbols.up
        },
        down: if range_shown.begin == max_begin {
            &scrollbar.symbols.down_disabled
        } else {
            &scrollbar.symbols.down
        },
        bar: &scrollbar.symbols.bar,
        space: &scrollbar.symbols.space,
    };

    // SPACE
    for y in (rect.y.0 + 1)..(rect.y.1) {
        queue!(
            stdout,
            cursor::MoveTo(rect.x.1, y),
            style::PrintStyledContent(StyledContent::new(symbols.space.style, symbols.space.char)),
        )
        .unwrap();
    }
    // UP
    queue!(
        stdout,
        cursor::MoveTo(rect.x.1, rect.y.0),
        style::PrintStyledContent(StyledContent::new(symbols.up.style, symbols.up.char)),
    )
    .unwrap();
    // BAR
    if rect.height() > 1 {
        let bar_ypos_value = {
            // ratio: 0 ~ 1.0
            let ratio = range_shown.begin as f32 / max_begin as f32;
            (ratio * (rect.height() - /*arrows*/2) as f32) + (rect.y.0 + /*up arrow*/1) as f32
        };
        queue!(
            stdout,
            cursor::MoveTo(rect.x.1, bar_ypos_value as u16),
            style::PrintStyledContent(StyledContent::new(symbols.bar.style, symbols.bar.char)),
        )
        .unwrap();
    }
    // DOWN
    queue!(
        stdout,
        cursor::MoveTo(rect.x.1, rect.y.1),
        style::PrintStyledContent(StyledContent::new(symbols.down.style, symbols.down.char)),
    )
    .unwrap();
}
