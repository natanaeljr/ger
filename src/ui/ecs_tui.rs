use crossterm::event::KeyModifiers;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    queue, style,
    terminal::{self, ClearType},
};
use legion::systems::CommandBuffer;
use legion::{component, Entity, IntoQuery, Resources, World};

use crate::ui::rect::Rect;
use crate::ui::table::{Columns, Selection, Table, VerticalScroll};
use crate::ui::term::{TermProps, TermUSize};
use crate::ui::winbox::WinBox;

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

        let mut resources = Resources::default();
        let mut commands = CommandBuffer::new(&self.registry);

        // Draw Tables
        let mut query = <(
            Entity,
            &Rect,
            &Table,
            &Columns,
            Option<&VerticalScroll>,
            Option<&Selection>,
            Option<&WinBox>,
        )>::query();
        for (_entity, rect, table, columns, vscroll, selected, winbox) in query.iter(&self.registry)
        {
            let components = (rect, table, columns, vscroll, selected, winbox);
            super::draw::draw_table(stdout, components);
        }

        // Cache Tables
        let mut query = <(
            Entity,
            &Rect,
            &Table,
            &Columns,
            Option<&VerticalScroll>,
            Option<&Selection>,
            Option<&WinBox>,
            &mut Children,
        )>::query();
        for (entity, rect, table, columns, vscroll, selected, winbox, children) in
            query.iter_mut(&mut self.registry)
        {
            let components = (rect, table, columns, vscroll, selected, winbox, children);
            super::draw::cache_table(entity, &mut commands, components);
        }
        commands.flush(&mut self.registry, &mut resources);

        // Print entities
        // queue!(stdout, cursor::MoveTo(0, 10)).unwrap();
        // let mut query = <(Entity, &Rect)>::query()
        //     .filter(component::<ColumnHeader>() | component::<ColumnSeparator>());
        // for (entity, rect) in query.iter_mut(&mut self.registry) {
        //     queue!(
        //         stdout,
        //         style::Print(format!("entity: {:?}, rect: {:?}, width: {}, height {}", entity, rect, rect.width(), rect.height())),
        //         cursor::MoveToNextLine(1),
        //     )
        //     .unwrap();
        // }

        // Print entities
        let mut entities : Vec<Entity> = Vec::new();
        queue!(stdout, cursor::MoveTo(0, 10)).unwrap();
        let mut query = <(Entity,)>::query();
        for (entity,) in query.iter(&self.registry) {
            entities.push(entity.clone());
        }

        for entt in &entities {
            let entry = self.registry.entry(entt.clone()).unwrap();
            queue!(
                stdout,
                style::Print(format!("{:?} has {:?}", entt, entry.archetype().layout().component_types())),
                cursor::MoveToNextLine(1),
            )
                .unwrap();
        }

        // let entry = self.registry.entry(self.registry.push(()))?.archetype().
    }

    fn is_canvas_drawable(&self) -> bool {
        self.term_cache.width >= 1 && self.term_cache.height >= 1
    }
}

pub struct ColumnSeparator {}

pub struct ColumnHeader {}

#[derive(Clone)]
#[repr(transparent)]
pub struct Parent(pub Entity);

#[repr(transparent)]
pub struct Children(pub Vec<Entity>);
