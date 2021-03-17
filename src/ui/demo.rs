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
    let mut row2 = Row::new();
    row2.insert(ChangeColumn::Commit as ColumnIndex, String::from("18d3290"));
    row2.insert(ChangeColumn::Number as ColumnIndex, String::from("104525"));
    row2.insert(
        ChangeColumn::Owner as ColumnIndex,
        String::from("Joao Begin"),
    );
    row2.insert(ChangeColumn::Time as ColumnIndex, String::from("07:16 PM"));
    row2.insert(
        ChangeColumn::Project as ColumnIndex,
        String::from("feature-center"),
    );
    row2.insert(ChangeColumn::Branch as ColumnIndex, String::from("future"));
    row2.insert(ChangeColumn::Topic as ColumnIndex, String::from("dial"));
    row2.insert(ChangeColumn::Status as ColumnIndex, String::from("NEW"));
    row2.insert(
        ChangeColumn::Subject as ColumnIndex,
        String::from("Add diagnostics feature to some platforms"),
    );
    let mut row3 = Row::new();
    row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("18d3290"));
    row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104525"));
    row3.insert(
        ChangeColumn::Owner as ColumnIndex,
        String::from("Joao Begin"),
    );
    row3.insert(ChangeColumn::Time as ColumnIndex, String::from("07:16 PM"));
    row3.insert(
        ChangeColumn::Project as ColumnIndex,
        String::from("feature-center"),
    );
    row3.insert(ChangeColumn::Branch as ColumnIndex, String::from("future"));
    row3.insert(ChangeColumn::Topic as ColumnIndex, String::from("dial"));
    row3.insert(ChangeColumn::Status as ColumnIndex, String::from("NEW"));
    row3.insert(
        ChangeColumn::Subject as ColumnIndex,
        String::from("Add diagnostics feature to some platforms"),
    );
    let table = Table {
        rows: vec![row, row2, row3],
    };
    let columns = Columns {
        print_header: true,
        visible: vec![
            Column {
                index: ChangeColumn::Commit as ColumnIndex,
                name: "commit".to_string(),
                width: 7,
                style: ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Number as ColumnIndex,
                name: "number".to_string(),
                width: 6,
                style: ContentStyle::new()
                    .foreground(Color::Yellow)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Right,
            },
            Column {
                index: ChangeColumn::Owner as ColumnIndex,
                name: "owner".to_string(),
                width: 17,
                style: ContentStyle::new()
                    .foreground(Color::DarkGrey)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Time as ColumnIndex,
                name: "time".to_string(),
                width: 10,
                style: ContentStyle::new()
                    .foreground(Color::Magenta)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Project as ColumnIndex,
                name: "project".to_string(),
                width: 30,
                style: ContentStyle::new()
                    .foreground(Color::Cyan)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Branch as ColumnIndex,
                name: "branch".to_string(),
                width: 20,
                style: ContentStyle::new()
                    .foreground(Color::DarkCyan)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Topic as ColumnIndex,
                name: "topic".to_string(),
                width: 20,
                style: ContentStyle::new()
                    .foreground(Color::DarkCyan)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
            Column {
                index: ChangeColumn::Status as ColumnIndex,
                name: "status".to_string(),
                width: 10,
                style: ContentStyle::new()
                    .foreground(Color::Green)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Center,
            },
            Column {
                index: ChangeColumn::Subject as ColumnIndex,
                name: "subject".to_string(),
                width: 50,
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
            },
        ],
        hidden: vec![],
        separator: '|',
    };
    let show_numbers = ShowNumbers::Normal;
    let rect = Rect::from_size((0, 0), (width, height));
    let components = (rect, table, columns, show_numbers);
    let _entity = registry.push(components);
}
