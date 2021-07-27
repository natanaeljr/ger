use crate::config::CliConfig;
use crate::handler::get_remote_restapi_handler;
use crate::ui::change::ChangeColumn;
use crate::ui::layout::{HorizontalAlignment, HorizontalMargin, LineNumberMode};
use crate::ui::rect::Rect;
use crate::ui::table::{
  resolve_line_number_column_width, Column, ColumnBuiltIn, ColumnIndex, ColumnValue, Columns, Row, Selection, Table,
  VerticalScroll,
};
use crate::ui::term::TermUSize;
use crate::ui::winbox::{BorderChars, BoxHint, WinBox};
use crate::util;
use crossterm::style::{Attribute, Color, ContentStyle};
use gerlib::changes::{AdditionalOpt, ChangeEndpoints, ChangeInfo, QueryParams};
use legion::{Entity, World};

pub fn create_table(config: &mut CliConfig, (width, height): (TermUSize, TermUSize), registry: &mut World) -> Entity {
  let mut row = Row::new();
  row.insert(ChangeColumn::Commit as ColumnIndex, String::from("8f524ac"));
  row.insert(ChangeColumn::Number as ColumnIndex, String::from("104508"));
  row.insert(ChangeColumn::Owner as ColumnIndex, String::from("Auto QA"));
  row.insert(ChangeColumn::Time as ColumnIndex, String::from("11:24 AM"));
  row.insert(ChangeColumn::Project as ColumnIndex, String::from("packet-system"));
  row.insert(ChangeColumn::Branch as ColumnIndex, String::from("develop"));
  row.insert(ChangeColumn::Status as ColumnIndex, String::from("NEW"));
  row.insert(
    ChangeColumn::Subject as ColumnIndex,
    String::from("Remove Conditional verification info"),
  );
  let mut row2 = Row::new();
  row2.insert(ChangeColumn::Commit as ColumnIndex, String::from("18d3290"));
  row2.insert(ChangeColumn::Number as ColumnIndex, String::from("104525"));
  row2.insert(ChangeColumn::Owner as ColumnIndex, String::from("Joao Begin"));
  row2.insert(ChangeColumn::Time as ColumnIndex, String::from("07:16 PM"));
  row2.insert(ChangeColumn::Project as ColumnIndex, String::from("feature-center"));
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
  row3.insert(ChangeColumn::Owner as ColumnIndex, String::from("Joao Begin"));
  row3.insert(ChangeColumn::Time as ColumnIndex, String::from("07:16 PM"));
  row3.insert(ChangeColumn::Project as ColumnIndex, String::from("feature-center"));
  row3.insert(ChangeColumn::Branch as ColumnIndex, String::from("future"));
  row3.insert(ChangeColumn::Topic as ColumnIndex, String::from("dial"));
  row3.insert(ChangeColumn::Status as ColumnIndex, String::from("ABANDONED"));
  row3.insert(
    ChangeColumn::Subject as ColumnIndex,
    String::from("Add diagnostics feature to some platforms"),
  );
  let mut table = Table {
    rows: vec![row, row2, row3],
  };
  table = real_table(config);
  // table.rows.extend(table.rows.clone());

  let columns = Columns {
    print_header: true,
    visible: vec![
      Column {
        name: "".to_string(),
        width: resolve_line_number_column_width(table.rows.len()),
        style: ContentStyle::new()
          .foreground(Color::Green)
          .attribute(Attribute::Dim)
          .attribute(Attribute::Bold),
        alignment: HorizontalAlignment::Right,
        value: ColumnValue::BuiltIn {
          builtin: ColumnBuiltIn::LineNumber {
            mode: LineNumberMode::Normal,
            style: ContentStyle::new().foreground(Color::Green).attribute(Attribute::Bold),
          },
        },
      },
      Column {
        name: "commit".to_string(),
        width: 7,
        style: ContentStyle::new()
          .attribute(Attribute::Bold)
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
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
          .attribute(Attribute::Reverse),
        alignment: HorizontalAlignment::Left,
        value: ColumnValue::Data {
          index: ChangeColumn::Subject as ColumnIndex,
        },
      },
    ],
    hidden: vec![],
    separator: ' ',
  };
  let vscroll = VerticalScroll { top_row: 0 };
  let selection = Selection {
    row_index: 3,
    style: ContentStyle::new().attribute(Attribute::Reverse),
  };
  let winbox = WinBox {
    style: ContentStyle::new().foreground(Color::Green),
    borders: BorderChars::simple().clone(),
    top_hints: vec![
      BoxHint {
        content: "change list".to_string(),
        style: ContentStyle::new(),
        margin: HorizontalMargin { left: 1, right: 0 },
        alignment: HorizontalAlignment::Left,
      },
      BoxHint {
        content: "(query)".to_string(),
        style: ContentStyle::new(),
        margin: HorizontalMargin { left: 0, right: 1 },
        alignment: HorizontalAlignment::Right,
      },
      BoxHint {
        content: "Hi".to_string(),
        style: ContentStyle::new(),
        margin: HorizontalMargin { left: 1, right: 0 },
        alignment: HorizontalAlignment::Center,
      },
      BoxHint {
        content: "-clock-".to_string(),
        style: ContentStyle::new(),
        margin: HorizontalMargin { left: 1, right: 1 },
        alignment: HorizontalAlignment::Center,
      },
      BoxHint {
        content: "Ho".to_string(),
        style: ContentStyle::new(),
        margin: HorizontalMargin { left: 0, right: 1 },
        alignment: HorizontalAlignment::Center,
      },
    ],
    bottom_hints: vec![
      BoxHint {
        content: "nothing".to_string(),
        style: ContentStyle::new()
          .attribute(Attribute::Dim)
          .attribute(Attribute::Reverse),
        margin: HorizontalMargin { left: 1, right: 0 },
        alignment: HorizontalAlignment::Left,
      },
      BoxHint {
        content: "[stats]".to_string(),
        style: ContentStyle::new()
          .attribute(Attribute::Dim)
          .attribute(Attribute::Reverse),
        margin: HorizontalMargin { left: 0, right: 1 },
        alignment: HorizontalAlignment::Right,
      },
      BoxHint {
        content: "ln".to_string(),
        style: ContentStyle::new().attribute(Attribute::Dim),
        margin: HorizontalMargin { left: 0, right: 0 },
        alignment: HorizontalAlignment::Right,
      },
      BoxHint {
        content: "#comment#".to_string(),
        style: ContentStyle::new(),
        margin: HorizontalMargin { left: 0, right: 0 },
        alignment: HorizontalAlignment::Center,
      },
    ],
  };
  let rect = Rect::from_size_unchecked((0, 0), (width, height));
  let components = (rect, winbox, table, columns, vscroll, selection);
  let entity = registry.push(components);
  entity

  // DEMO CONSOLE WINDOW:
  // let rect2 = Rect::from_size_unchecked((0, height - 3), (width, 3));
  // let winbox2 = WinBox {
  //   style: Default::default(),
  //   borders: BorderChars::simple().clone(),
  //   top_hints: vec![BoxHint {
  //     content: "console".to_string(),
  //     style: ContentStyle::new().attribute(Attribute::Reverse),
  //     margin: HorizontalMargin { left: 1, right: 1 },
  //     alignment: HorizontalAlignment::Left,
  //   }],
  //   bottom_hints: vec![],
  // };
  // let components = (rect2, winbox2);
  // let _entity = registry.push(components);
}

pub fn real_table(config: &mut CliConfig) -> Table {
  let mut rest = get_remote_restapi_handler(config, None).unwrap();
  let query_param = QueryParams {
    search_queries: None,
    additional_opts: Some(vec![AdditionalOpt::DetailedAccounts, AdditionalOpt::CurrentRevision]),
    limit: Some(50),
    start: None,
  };
  let changes_list: Vec<Vec<ChangeInfo>> = rest.query_changes(&query_param).unwrap();

  if changes_list.is_empty() {
    // writeln!(config.stdout, "No changes.")?;
    return Table::default();
  }

  let mut rows = Vec::new();
  for changes in &changes_list {
    for change in changes {
      let mut row = Row::new();
      if let Some(current_revision) = change.current_revision.as_ref() {
        row.insert(ChangeColumn::Commit as ColumnIndex, current_revision[..7].to_owned());
      }
      row.insert(ChangeColumn::Number as ColumnIndex, change.number.to_string());
      row.insert(
        ChangeColumn::Owner as ColumnIndex,
        change.owner.name.as_ref().unwrap().clone(),
      );
      row.insert(
        ChangeColumn::Time as ColumnIndex,
        util::format_short_datetime(&change.updated.0),
      );
      row.insert(ChangeColumn::Project as ColumnIndex, change.project.clone());
      row.insert(ChangeColumn::Branch as ColumnIndex, change.branch.clone());
      row.insert(ChangeColumn::Status as ColumnIndex, change.status.to_string());
      row.insert(ChangeColumn::Subject as ColumnIndex, change.subject.clone());
      rows.push(row);
    }
  }
  Table { rows }
}
