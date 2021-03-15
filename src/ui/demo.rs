use crate::ui::change::ChangeColumn;
use crate::ui::layout::{HorizontalAlignment, ShowNumbers};
use crate::ui::r#box::Rect;
use crate::ui::table::{Column, ColumnIndex, Columns, Row, Table};
use crate::ui::term::TermUSize;
use crossterm::style::{Attribute, Color, ContentStyle};
use legion::World;

pub fn create_table((width, height): (TermUSize, TermUSize), registry: &mut World) {
    let mut row = Row::new();
    row.insert(ChangeColumn::Commit as ColumnIndex, String::from("8f524ac"));
    row.insert(ChangeColumn::Number as ColumnIndex, String::from("104508"));
    row.insert(ChangeColumn::Owner as ColumnIndex, String::from("Auto QA"));
    row.insert(ChangeColumn::Time as ColumnIndex, String::from("11:24 AM"));
    row.insert(
        ChangeColumn::Project as ColumnIndex,
        String::from("packet-system"),
    );
    row.insert(ChangeColumn::Branch as ColumnIndex, String::from("develop"));
    row.insert(ChangeColumn::Status as ColumnIndex, String::from("NEW"));
    row.insert(
        ChangeColumn::Subject as ColumnIndex,
        String::from("Remove Conditional verification info"),
    );
    let table = Table { data: vec![row] };
    let columns = Columns {
        print_header: true,
        visible: vec![
            Column {
                index: ChangeColumn::Commit as ColumnIndex,
                name: "commit".to_string(),
                len: 8,
                style: ContentStyle::new().attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Number as ColumnIndex,
                name: "number".to_string(),
                len: 8,
                style: ContentStyle::new()
                    .foreground(Color::Yellow)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Owner as ColumnIndex,
                name: "owner".to_string(),
                len: 17,
                style: ContentStyle::new()
                    .foreground(Color::DarkGrey)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Time as ColumnIndex,
                name: "time".to_string(),
                len: 10,
                style: ContentStyle::new()
                    .foreground(Color::Magenta)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Project as ColumnIndex,
                name: "project".to_string(),
                len: 30,
                style: ContentStyle::new()
                    .foreground(Color::Cyan)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Branch as ColumnIndex,
                name: "branch".to_string(),
                len: 20,
                style: ContentStyle::new()
                    .foreground(Color::DarkCyan)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Topic as ColumnIndex,
                name: "topic".to_string(),
                len: 20,
                style: ContentStyle::new()
                    .foreground(Color::DarkCyan)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Status as ColumnIndex,
                name: "status".to_string(),
                len: 10,
                style: ContentStyle::new()
                    .foreground(Color::Green)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Subject as ColumnIndex,
                name: "subject".to_string(),
                len: 50,
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Left,
            },
        ],
        hidden: vec![],
    };
    let show_numbers = ShowNumbers::Normal;
    let rect = Rect::from_size((0, 0), (width, height));
    let components = (rect, table, columns, show_numbers);
    let _entity = registry.push(components);
}
