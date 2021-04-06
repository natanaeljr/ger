use crate::ui::change::ChangeColumn;
use crate::ui::layout::{HorizontalAlignment, LineNumberMode};
use crate::ui::rect::Rect;
use crate::ui::table::{
    resolve_line_number_column_width, Column, ColumnBuiltIn, ColumnIndex, ColumnValue, Columns,
    Row, Selection, Table, VerticalScroll,
};
use crate::ui::term::TermUSize;
use crate::ui::winbox::{BorderChars, WinBox};
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
    row2.insert(ChangeColumn::Status as ColumnIndex, String::from("MERGED"));
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
    row3.insert(
        ChangeColumn::Status as ColumnIndex,
        String::from("ABANDONED"),
    );
    row3.insert(
        ChangeColumn::Subject as ColumnIndex,
        String::from("Add diagnostics feature to some platforms"),
    );
    let mut table = Table {
        rows: vec![row, row2, row3],
    };
    table.rows.extend(table.rows.clone());
    let columns = Columns {
        print_header: true,
        visible: vec![
            Column {
                name: "".to_string(),
                width: resolve_line_number_column_width(table.rows.len()),
                style: ContentStyle::new()
                    .foreground(Color::Green)
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Underlined)
                    .attribute(Attribute::Bold),
                alignment: HorizontalAlignment::Right,
                value: ColumnValue::BuiltIn {
                    builtin: ColumnBuiltIn::LineNumber {
                        mode: LineNumberMode::Normal,
                        style: ContentStyle::new()
                            .foreground(Color::Green)
                            .attribute(Attribute::Bold),
                    },
                },
            },
            Column {
                name: "commit".to_string(),
                width: 7,
                style: ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Commit as ColumnIndex,
                },
            },
            Column {
                name: "number".to_string(),
                width: 6,
                style: ContentStyle::new()
                    .foreground(Color::Yellow)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Right,
                value: ColumnValue::Data {
                    index: ChangeColumn::Number as ColumnIndex,
                },
            },
            Column {
                name: "owner".to_string(),
                width: 17,
                style: ContentStyle::new()
                    .foreground(Color::DarkGrey)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Owner as ColumnIndex,
                },
            },
            Column {
                name: "time".to_string(),
                width: 10,
                style: ContentStyle::new()
                    .foreground(Color::Magenta)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Time as ColumnIndex,
                },
            },
            Column {
                name: "project".to_string(),
                width: 30,
                style: ContentStyle::new()
                    .foreground(Color::Cyan)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Project as ColumnIndex,
                },
            },
            Column {
                name: "branch".to_string(),
                width: 20,
                style: ContentStyle::new()
                    .foreground(Color::DarkCyan)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Branch as ColumnIndex,
                },
            },
            Column {
                name: "topic".to_string(),
                width: 20,
                style: ContentStyle::new()
                    .foreground(Color::DarkCyan)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Topic as ColumnIndex,
                },
            },
            Column {
                name: "status".to_string(),
                width: 10,
                style: ContentStyle::new()
                    .foreground(Color::Green)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Center,
                value: ColumnValue::Data {
                    index: ChangeColumn::Status as ColumnIndex,
                },
            },
            Column {
                name: "subject".to_string(),
                width: 50,
                style: ContentStyle::new()
                    .attribute(Attribute::Dim)
                    .attribute(Attribute::Bold)
                    .attribute(Attribute::Underlined),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Subject as ColumnIndex,
                },
            },
        ],
        hidden: vec![],
        separator: '|',
    };
    let vscroll = VerticalScroll { top_row: 0 };
    let selection = Selection {
        row_index: 3,
        style: ContentStyle::new().attribute(Attribute::Reverse),
    };
    let winbox = WinBox {
        style: ContentStyle::new().foreground(Color::Green),
        borders: BorderChars::simple().clone(),
    };
    let rect = Rect::from_size_unchecked((0, 0), (width, height));
    let components = (rect, winbox, table, columns, vscroll, selection);
    let _entity = registry.push(components);
}
