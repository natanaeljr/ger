use crossterm::style::{Attribute, ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};
use std::collections::HashSet;

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
        style: ContentStyle::new(),
      },
      up_disabled: CharStyle {
        char: '↑',
        style: ContentStyle::new().attribute(Attribute::Dim).attribute(Attribute::Bold),
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
        style: ContentStyle::new(),
      },
      down_disabled: CharStyle {
        char: '↓',
        style: ContentStyle::new().attribute(Attribute::Dim).attribute(Attribute::Bold),
      },
      space: CharStyle {
        char: ' ',
        style: ContentStyle::new(),
      },
      space_clicked: CharStyle {
        char: ' ',
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
        style: ContentStyle::new(),
      },
      up_disabled: CharStyle {
        char: '^',
        style: ContentStyle::new().attribute(Attribute::Dim).attribute(Attribute::Bold),
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
        style: ContentStyle::new(),
      },
      down_disabled: CharStyle {
        char: 'v',
        style: ContentStyle::new().attribute(Attribute::Dim).attribute(Attribute::Bold),
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

pub struct ScrollBar {
  pub x: u16,
  pub y: u16,
  pub height: u16,
  pub symbols: ScrollBarChars,
}

pub struct RangeTotal {
  pub begin: usize,
  pub end: usize,
  pub total: usize,
}

impl ScrollBar {
  pub fn draw<W>(&self, stdout: &mut W, range_shown: RangeTotal, bar_clicking: bool, up_arrow_click: bool, down_arrow_click: bool)
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
    }
    let symbols = Symbols {
      up: if range_shown.begin == 0 {
        &self.symbols.up_disabled
      } else if up_arrow_click {
        &self.symbols.up_clicked
      } else {
        &self.symbols.up
      },
      down: if range_shown.begin == max_begin {
        &self.symbols.down_disabled
      } else if down_arrow_click {
        &self.symbols.down_clicked
      } else {
        &self.symbols.down
      },
      bar: if bar_clicking { &self.symbols.bar_clicked } else { &self.symbols.bar },
    };

    // SPACE
    for y in (self.y + 1)..(self.y + self.height) {
      queue!(
        stdout,
        cursor::MoveTo(self.x, y),
        style::PrintStyledContent(StyledContent::new(self.symbols.space.style, self.symbols.space.char)),
      )
      .unwrap();
    }
    if bar_clicking {
      for y in Self::positions(self.height, self.y, &range_shown).iter() {
        queue!(
          stdout,
          cursor::MoveTo(self.x, *y),
          style::PrintStyledContent(StyledContent::new(self.symbols.space_clicked.style, self.symbols.space_clicked.char)),
        )
        .unwrap();
      }
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

  pub fn bar_ypos(height: u16, y: u16, range_shown: &RangeTotal) -> u16 {
    let visible_count = range_shown.end - range_shown.begin;
    if visible_count >= (range_shown.total) {
      return 0;
    }
    let max_begin = range_shown.total - (visible_count);
    let bar_ypos_value = {
      // ratio: 0 ~ 1.0
      let ratio = range_shown.begin as f32 / max_begin as f32;
      (ratio * (height - /*arrows*/2) as f32) + (y + /*up arrow*/1) as f32
    };
    bar_ypos_value as u16
  }

  pub fn scroll(height: u16, y: u16, range_shown: &RangeTotal, bar_ypos: u16) -> u16 {
    let visible_count = range_shown.end - range_shown.begin;
    if visible_count >= (range_shown.total) {
      return 0;
    }
    let max_begin = range_shown.total - (visible_count);
    let ratio = (bar_ypos as f32 - (y + /*up arrow*/1) as f32) / (height - /*arrows*/2) as f32;
    let row = (ratio * max_begin as f32).round();
    row as u16
  }

  pub fn positions(height: u16, y: u16, range_shown: &RangeTotal) -> HashSet<u16> {
    let visible_count = range_shown.end - range_shown.begin;
    if visible_count >= (range_shown.total) {
      return HashSet::new();
    };
    let max_begin = range_shown.total - (visible_count);
    let mut set = HashSet::new();
    // Note: probably there is better math for this
    for row in 0..=max_begin {
      let ypos = {
        // ratio: 0 ~ 1.0
        let ratio = row as f32 / max_begin as f32;
        (ratio * (height - /*arrows*/2) as f32) + (y + /*up arrow*/1) as f32
      };
      set.insert(ypos as u16);
    }
    set
  }
}
