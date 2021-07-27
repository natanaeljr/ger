use crate::config::CliConfig;
use crate::ui::change::ChangeColumn;
use crate::ui::layout::HorizontalAlignment;
use crate::ui::rect::Rect;
use crate::ui::table::{ColumnIndex, Columns, Selection, Table, VerticalScroll};
use crate::ui::term::{TermProps, TermUSize};
use crate::ui::winbox::{BorderChars, BoxHint, WinBox};
use crossterm::event::KeyModifiers;
use crossterm::style::{Color, ContentStyle};
use crossterm::{
  event::{self, Event, KeyCode},
  queue, style,
  terminal::{self, ClearType},
};
use legion::{component, Entity, EntityStore, IntoQuery, World};
use log::trace;
use std::str::FromStr;

/// Entity Component System
/// Terminal User Interface
pub struct EcsTui {
  term_cache: TermProps,
  registry: World,
  selected_window: Option<Entity>,
}

impl EcsTui {
  pub fn new(config: &mut CliConfig) -> Self {
    let (width, height) = terminal::size().unwrap();
    let mut this = Self {
      term_cache: TermProps { width, height },
      registry: World::default(),
      selected_window: None,
    };
    let table = super::demo::create_table(config, (width, height), &mut this.registry);
    this.selected_window = Some(table);
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
              KeyCode::Char('l') => {
                trace!("Went here");
                if let Some(window_entt) = self.selected_window {
                  let mut maybe_right_rect = None;
                  let mut maybe_number = None;
                  {
                    trace!("Not here");
                    let mut window_entry = self.registry.entry_mut(window_entt).unwrap();

                    let window_rect = window_entry.get_component_mut::<Rect>().unwrap();
                    let (left_rect, right_rect) = window_rect.vsplit().unwrap();
                    maybe_right_rect = Some(right_rect);
                    *window_rect = left_rect;

                    let selected_row_index = window_entry.get_component::<Selection>().unwrap().row_index;
                    let window_table = window_entry.get_component_mut::<Table>().unwrap();
                    let number = window_table.rows[selected_row_index]
                      .get(&(ChangeColumn::Number as ColumnIndex))
                      .unwrap();
                    let number = <u32>::from_str(number.as_str()).unwrap();
                    maybe_number = Some(number);
                  }
                  if let Some(right_rect) = maybe_right_rect {
                    let winbox = WinBox {
                      style: ContentStyle::new().foreground(Color::Cyan),
                      borders: BorderChars::simple().clone(),
                      top_hints: vec![BoxHint {
                        content: format!("Change {}", maybe_number.unwrap()),
                        style: Default::default(),
                        margin: Default::default(),
                        alignment: HorizontalAlignment::Left,
                      }],
                      bottom_hints: vec![],
                    };
                    self.registry.push((right_rect, winbox));
                  }
                }
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
    let mut query = <(
      &Rect,
      &Table,
      &Columns,
      Option<&VerticalScroll>,
      Option<&Selection>,
      Option<&WinBox>,
    )>::query();
    for (rect, table, columns, vscroll, selected, winbox) in query.iter(&self.registry) {
      super::draw::draw_table(stdout, (rect, table, columns, vscroll, selected, winbox));
    }

    let mut query = <(&Rect, &WinBox)>::query().filter(!component::<Table>());
    for (rect, winbox) in query.iter(&self.registry) {
      super::draw::draw_winbox(stdout, (rect, winbox));
    }
  }

  fn is_canvas_drawable(&self) -> bool {
    self.term_cache.width >= 1 && self.term_cache.height >= 1
  }
}
