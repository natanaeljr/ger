use crate::config::CliConfig;
use crate::handler::get_remote_restapi_handler;
use crate::ui::change::ChangeColumn;
use crate::ui::ecs_tui::Context;
use crate::ui::layout::{HorizontalAlignment, HorizontalMargin, LineNumberMode};
use crate::ui::rect::Rect;
use crate::ui::table;
use crate::ui::table::{
  Column, ColumnBuiltIn, ColumnIndex, ColumnValue, Columns, Row, Selection, Table, VerticalScroll,
};
use crate::ui::term::TermUSize;
use crate::ui::winbox::{BorderChars, BoxHint, WinBox};
use crate::{ui, util};
use crossterm::style::{Attribute, Color, ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};
use gerlib::changes::{AdditionalOpt, ChangeEndpoints, ChangeInfo, FileStatus, QueryParams};
use legion::{Entity, EntityStore, IntoQuery, World};
use std::io::Write;
use std::str::FromStr;
use termcolor::BufferWriter;

pub fn initialize_windows(config: &mut CliConfig, registry: &mut World, context: &mut Context) {
  let table = push_table_window(config, (context.term_cache.width, context.term_cache.height), registry);
  context.wm.selected_window = Some(table);
}

pub fn list_changes(config: &mut CliConfig) -> Vec<Vec<ChangeInfo>> {
  let mut rest = get_remote_restapi_handler(config, None).unwrap();
  let query_param = QueryParams {
    search_queries: None,
    additional_opts: Some(vec![AdditionalOpt::DetailedAccounts, AdditionalOpt::CurrentRevision]),
    limit: Some(50),
    start: None,
  };
  let changes_list: Vec<Vec<ChangeInfo>> = rest.query_changes(&query_param).unwrap();
  changes_list
}

pub fn get_change(config: &mut CliConfig, number: u32) -> Option<ChangeInfo> {
  let mut rest = get_remote_restapi_handler(config, None).unwrap();
  let additional_opts = vec![
    AdditionalOpt::CurrentRevision,
    AdditionalOpt::CurrentCommit,
    AdditionalOpt::CurrentFiles,
    AdditionalOpt::DetailedAccounts,
    AdditionalOpt::DetailedLabels,
  ];
  let change: ChangeInfo = rest.get_change(&number.to_string(), Some(additional_opts)).unwrap();
  Some(change)
}

pub fn push_changes(registry: &mut World, changes_list: Vec<Vec<ChangeInfo>>) -> Vec<Entity> {
  let mut entts = Vec::new();
  for changes in changes_list {
    entts.reserve(changes.len());
    for change in changes {
      let entt = registry.push((change,));
      entts.push(entt);
    }
  }
  entts
}

pub fn open_change_window(registry: &mut World, context: &mut Context, config: &mut CliConfig) -> bool {
  if let Some(window_entt) = context.wm.selected_window {
    let mut maybe_right_rect = None;
    let mut maybe_number = None;
    {
      let mut window_entry = registry.entry_mut(window_entt).unwrap();

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
      let change_info = Some((0, get_change(config, 101249).unwrap()));
      // let change_info = search_change_info_by_number(registry, maybe_number.unwrap());
      if let Some((_entt, change_info)) = change_info {
        let change = change_info.clone();
        push_change_info_window(registry, right_rect, change);
      }
    }
    return true;
  }
  false
}

pub fn search_change_info_by_number(registry: &World, number: u32) -> Option<(Entity, &ChangeInfo)> {
  let mut query = <(Entity, &ChangeInfo)>::query();
  for (entt, change) in query.iter(registry) {
    let change: &ChangeInfo = change;
    if change.number == number {
      return Some((*entt, change));
    }
  }
  None
}

pub fn push_change_info_window(registry: &mut World, rect: Rect, change: ChangeInfo) {
  let winbox = WinBox {
    style: ContentStyle::new().foreground(Color::Cyan),
    borders: BorderChars::simple().clone(),
    top_hints: vec![BoxHint {
      content: format!("Change {}", change.number),
      style: Default::default(),
      margin: Default::default(),
      alignment: HorizontalAlignment::Left,
    }],
    bottom_hints: vec![],
  };
  registry.push((rect, winbox, change));
}

pub fn write_out_change_info(change: &ChangeInfo) -> String {
  let mut buffer = Vec::new();
  write!(buffer, "Change {}", change.number);

  write!(buffer, " - {}", change.status);
  if change.work_in_progress {
    write!(buffer, " (WIP)");
  }

  if let Some(total_comment_count) = change.total_comment_count {
    write!(buffer, " > comments {}", total_comment_count);
    if let Some(unresolved_comment_count) = change.unresolved_comment_count {
      write!(buffer, " [new: {}]", unresolved_comment_count);
    }
  }

  let current_revision = change.current_revision.as_ref();
  let current_revision_info = change
    .revisions
    .as_ref()
    .and_then(|revisions| current_revision.and_then(|current_revision| revisions.get(current_revision)));
  let current_commit = current_revision_info.and_then(|curr_rev_info| curr_rev_info.commit.as_ref());
  let patch_set = current_revision_info.and_then(|curr_rev_info| Some(curr_rev_info._number));

  if let Some(patch_set) = patch_set {
    write!(buffer, "  [patch set: {}/{}]", patch_set, patch_set);
  }

  write!(buffer, "\n");

  write!(buffer, "Owner:       ");
  if let Some(owner_name) = &change.owner.name {
    write!(buffer, "{}", owner_name);
  } else if let Some(owner_username) = &change.owner.username {
    write!(buffer, "{}", owner_username);
  } else {
    write!(buffer, "({})", &change.owner.account_id);
  }
  if let Some(owner_email) = &change.owner.email {
    write!(buffer, " <{}>", owner_email);
  }
  write!(buffer, "\n");

  writeln!(buffer, "Updated:     {}", util::format_long_datetime(&change.updated.0));

  writeln!(buffer, "Project:     {}", change.project);

  writeln!(buffer, "Branch:      {}", change.branch);

  if let Some(topic) = &change.topic {
    writeln!(buffer, "Topic:       {}", topic);
  }

  if let Some(current_commit) = current_commit {
    if let Some(author) = &current_commit.author {
      writeln!(buffer, "Author:      {} <{}>", author.name, author.email);
    }
    if let Some(committer) = &current_commit.committer {
      writeln!(buffer, "Committer:   {} <{}>", committer.name, committer.email);
    }
  }

  if let Some(current_revision) = current_revision {
    writeln!(buffer, "Commit:      {}", current_revision);
  }

  if let Some(strategy) = &change.submit_type {
    writeln!(buffer, "Strategy:    {}", strategy);
  }

  if let Some(labels) = &change.labels {
    let mut label_maxlen = 0;
    for label in labels {
      if label.0.len() > label_maxlen {
        label_maxlen = label.0.len();
      }
    }

    for label in labels {
      let mut max = 0;
      let mut min = 0;
      if let Some(values) = &label.1.values {
        for value in values {
          let value: i32 = value.0.trim().parse().unwrap();
          if value > max {
            max = value;
          }
          if value < min {
            min = value;
          }
        }
      }

      let mut padding = label_maxlen - label.0.len();

      write!(buffer, "{}:", label.0);
      let mut no_vote = true;

      if let Some(label_all) = &label.1.all {
        for approval in label_all {
          if let Some(value) = approval.value {
            if value != 0 {
              no_vote = false;

              if padding > 0 {
                write!(buffer, "{0:1$}", ' ', padding);
              }
              write!(buffer, " {:+}", value);

              if let Some(name) = &approval.account.name {
                write!(buffer, " {}", name);
              }
              if let Some(email) = &approval.account.email {
                write!(buffer, " <{}>", email);
              }

              write!(buffer, "\n");
              padding = label_maxlen + 1;
            }
          }
        }
      } else {
        if padding > 0 {
          write!(buffer, "{0:1$}", ' ', padding);
        }
        write!(buffer, " -");
      }

      if no_vote {
        write!(buffer, "\n");
      }
    }

    if !labels.is_empty() {
      write!(buffer, "\n");
    }
  }

  if let Some(current_commit) = current_commit {
    if let Some(message) = &current_commit.message {
      let lines = message.lines();
      for line in lines {
        writeln!(buffer, "    {}", line);
      }
      write!(buffer, "\n");
    }
  }

  let current_files = current_revision_info.and_then(|cri| cri.files.as_ref());

  if let Some(current_files) = current_files {
    if !current_files.is_empty() {
      writeln!(buffer, "Files:");
    }

    let mut file_maxlen = 0;
    for file in current_files {
      if file.0.len() > file_maxlen {
        file_maxlen = file.0.len();
      }
    }

    if !current_files.is_empty() {
      //        let mut total_lines_inserted = 0;
      //        let mut total_lines_deleted = 0;

      for file in current_files {
        write!(buffer, " {}", file_status_initial(&file.1.status));

        write!(buffer, " {}", file.0,);

        let padding = file_maxlen - file.0.len();
        if padding > 0 {
          write!(buffer, "{0:1$}", ' ', padding);
        }
        write!(buffer, " |");

        if let Some(lines_inserted) = file.1.lines_inserted {
          //                total_lines_inserted += lines_inserted;
          write!(buffer, " +");
          write!(buffer, "{}", lines_inserted);
        }

        if let Some(lines_deleted) = file.1.lines_deleted {
          //                total_lines_deleted += lines_deleted;
          write!(buffer, " -");
          write!(buffer, "{}", lines_deleted);
        }

        write!(buffer, "\n");
      }

      //        let file_s = if current_files.len() > 1 { "s" } else { "" };
      //        let total_str = format!(" total {} file{} changed", current_files.len(), file_s);
      //        write!(stdout, "{}", total_str);
      //
      //        let padding = file_maxlen - total_str.len() + 3;
      //        if padding > 0 {
      //            write!(stdout, "{0:1$}", ' ', padding);
      //        }
      //        stdout.write_all(b" |")?;
      //
      //        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
      //        stdout.write_all(b" +")?;
      //        stdout.reset()?;
      //        write!(stdout, "{}", total_lines_inserted);
      //
      //        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
      //        stdout.write_all(b" -")?;
      //        stdout.reset()?;
      //        write!(stdout, "{}", total_lines_deleted);
      //
      //        stdout.write_all(b"\n")?;
    }
  }

  String::from_utf8(buffer).unwrap()
}

fn file_status_initial(status: &FileStatus) -> char {
  match status {
    FileStatus::Added => 'A',
    FileStatus::Modified => 'M',
    FileStatus::Deleted => 'D',
    FileStatus::Renamed => 'R',
    FileStatus::Copied => 'C',
    FileStatus::Rewritten => 'W',
  }
}

pub fn draw_change_info_window<W>(stdout: &mut W, (rect, winbox, change): (&Rect, &WinBox, &ChangeInfo)) -> Option<()>
where
  W: std::io::Write,
{
  super::draw::draw_winbox(stdout, (rect, winbox));
  let rect = rect.inner()?;

  queue!(stdout, cursor::MoveTo(rect.x.0 + 1, rect.y.0)).unwrap();

  let buffer = write_out_change_info(change);
  for line in buffer.lines() {
    queue!(stdout, style::Print(line)).unwrap();
    queue!(stdout, cursor::MoveToNextLine(1), cursor::MoveToColumn(rect.x.0 + 2)).unwrap();
  }

  Some(())
}

pub fn create_table_component(changes_list: &Vec<Vec<ChangeInfo>>) -> Table {
  let mut rows = Vec::new();
  for changes in changes_list {
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

pub fn push_table_window(
  config: &mut CliConfig, (width, height): (TermUSize, TermUSize), registry: &mut World,
) -> Entity {
  let changes_list = list_changes(config);
  let table = create_table_component(&changes_list);
  let _change_entts = push_changes(registry, changes_list);
  let columns = get_change_table_columns(&table);
  let vscroll = VerticalScroll { top_row: 0 };
  let selection = Selection {
    row_index: 3,
    style: ContentStyle::new().attribute(Attribute::Reverse),
  };
  let winbox = get_change_table_winbox();
  let rect = Rect::from_size_unchecked((0, 0), (width, height));
  let components = (rect, winbox, table, columns, vscroll, selection);
  let entity = registry.push(components);
  entity
}

pub fn get_change_table_columns(table: &Table) -> Columns {
  let columns = Columns {
    print_header: true,
    visible: vec![
      Column {
        name: "".to_string(),
        width: ui::table::resolve_line_number_column_width(table.rows.len()),
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
  columns
}

pub fn get_change_table_winbox() -> WinBox {
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
  winbox
}
