use crate::config::CliConfig;
use crate::ui::change::ChangeColumn;
use crate::ui::layout::HorizontalAlignment;
use crate::ui::main;
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
use gerlib::changes::ChangeInfo;
use legion::{component, Entity, EntityStore, IntoQuery, World};
use log::trace;
use std::str::FromStr;

pub struct WindowManager {
  pub selected_window: Option<Entity>,
}

pub struct Context {
  pub term_cache: TermProps,
  pub wm: WindowManager,
}

/// Entity Component System
/// Terminal User Interface
pub struct EcsTui {
  registry: World,
  context: Context,
}

impl EcsTui {
  pub fn new(config: &mut CliConfig) -> Self {
    let (width, height) = terminal::size().unwrap();
    let mut this = Self {
      registry: World::default(),
      context: Context {
        term_cache: TermProps { width, height },
        wm: WindowManager { selected_window: None },
      },
    };
    main::initialize_windows(config, &mut this.registry, &mut this.context);
    this
  }

  pub fn main_loop<W>(&mut self, stdout: &mut W, config: &mut CliConfig)
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
      self.event_loop(&mut quit, config);
      if quit {
        break;
      }
    }
  }

  fn event_loop(&mut self, quit: &mut bool, config: &mut CliConfig) {
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
                if main::open_change_window(&mut self.registry, &mut self.context, config) {
                  break;
                }
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
    self.context.term_cache.width = cols;
    self.context.term_cache.height = rows;
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

    let mut query = <(&Rect, &WinBox, &ChangeInfo)>::query();
    for (rect, winbox, change) in query.iter(&self.registry) {
      // super::draw::draw_winbox(stdout, (rect, winbox));
      main::draw_change_info_window(stdout, (rect, winbox, change));
    }
  }

  fn is_canvas_drawable(&self) -> bool {
    self.context.term_cache.width >= 1 && self.context.term_cache.height >= 1
  }
}
