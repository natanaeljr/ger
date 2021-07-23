use crate::ui::rect::Rect;
use crate::ui::table::{Columns, Selection, Table, VerticalScroll};
use crate::ui::term::{TermProps, TermUSize};
use crate::ui::winbox::WinBox;
use crossterm::event::KeyModifiers;
use crossterm::{
  event::{self, Event, KeyCode},
  queue, style,
  terminal::{self, ClearType},
};
use legion::{IntoQuery, World};

/// Entity Component System
/// Terminal User Interface
pub struct EcsTui {
  term_cache: TermProps,
  registry: World,
}

impl EcsTui {
  pub fn new() -> Self {
    let (width, height) = terminal::size().unwrap();
    let mut this = Self {
      term_cache: TermProps { width, height },
      registry: World::default(),
    };
    super::demo::create_table((width, height), &mut this.registry);
    this
  }

  pub fn main_loop<W>(&mut self, stdout: &mut W)
  where
    W: std::io::Write,
  {
    loop {
      // Rendering
      queue!(stdout, style::ResetColor, terminal::Clear(ClearType::All)).unwrap();
      self.draw(stdout);
      stdout.flush().unwrap();
      // Event handling
      let mut quit = false;
      self.event_loop(&mut quit);
      if quit {
        break;
      }
    }
  }

  fn event_loop(&mut self, quit: &mut bool) {
    loop {
      match event::read().unwrap() {
        Event::Key(key) => {
          if key.modifiers == KeyModifiers::empty() {
            match key.code {
              KeyCode::Char('q') => {
                *quit = true;
                break;
              }
              _ => {}
            }
          }
        }
        Event::Mouse(_) => {}
        Event::Resize(cols, rows) => {
          self.resize(cols, rows);
          break;
        }
      }
    }
  }

  fn resize(&mut self, cols: TermUSize, rows: TermUSize) {
    self.term_cache.width = cols;
    self.term_cache.height = rows;
    if !self.is_canvas_drawable() {
      return;
    }
    let mut query = <&mut Rect>::query();
    for rect in query.iter_mut(&mut self.registry) {
      // We are dealing with only ONE entity for now
      // So resize it to fill up the entire screen
      *rect = Rect::from_size_unchecked((0, 0), (cols, rows));
    }
  }

  fn draw<W>(&mut self, stdout: &mut W)
  where
    W: std::io::Write,
  {
    if !self.is_canvas_drawable() {
      return;
    }
    // Draw Tables
    let mut query = <(&Rect, &Table, &Columns, Option<&VerticalScroll>, Option<&Selection>, Option<&WinBox>)>::query();
    for (rect, table, columns, vscroll, selected, winbox) in query.iter(&self.registry) {
      super::draw::draw_table(stdout, (rect, table, columns, vscroll, selected, winbox));
    }
  }

  fn is_canvas_drawable(&self) -> bool {
    self.term_cache.width >= 1 && self.term_cache.height >= 1
  }
}
