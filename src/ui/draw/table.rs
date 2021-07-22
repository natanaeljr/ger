use crossterm::style::{ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};
use legion::systems::CommandBuffer;
use legion::Entity;

use crate::ui::ecs_tui::{Children, ColumnHeader, ColumnSeparator, Parent};
use crate::ui::layout::LineNumberMode;
use crate::ui::rect::Rect;
use crate::ui::table::{
    Column, ColumnBuiltIn, ColumnValue, Columns, Row, Selection, Table, VerticalScroll,
};
use crate::ui::winbox::WinBox;

pub fn cache_table(
    entity: &Entity, commands: &mut CommandBuffer,
    (rect, _table, columns, _vscroll, _selection, winbox, children): (
        &Rect,
        &Table,
        &Columns,
        Option<&VerticalScroll>,
        Option<&Selection>,
        Option<&WinBox>,
        &mut Children,
    ),
) -> Option<()> {
    // Clone the rect as mut because we need to shrink it later to draw sub-parts of the table
    let mut rect = rect.clone();
    // Draw the Window Box borders
    if let Some(_) = winbox {
        rect = rect.inner()?;
    }

    for child in &children.0 {
        commands.remove(child.clone());
    }
    children.0.clear();

    // Print the column headers
    if columns.print_header {
        cache_table_headers(entity, commands, (&rect, columns, children));
        // rect = rect.offset_y0(1)?;
    }
    // Print table data rows
    // cache_table_rows((&rect, table, columns, vscroll, selection));
    // Successful Draw
    Some(())
}

fn cache_table_headers(
    table: &Entity, commands: &mut CommandBuffer,
    (rect, columns, children): (&Rect, &Columns, &mut Children),
) {
    let mut walked_col = 0;
    let cache_column_header =
        &mut |_column: &Column, column_separator: &str, available_column_width: usize| {
            let parent = Parent(table.clone());

            if column_separator.len() > 0 {
                let x = rect.x.0 + walked_col as u16;
                let separator_rect = Rect::from_size((x, rect.y.0), (1, 1)).unwrap();
                walked_col += column_separator.len();
                let separator_entt =
                    commands.push((ColumnSeparator {}, parent.clone(), separator_rect));
                children.0.push(separator_entt);
            }

            if available_column_width > 0 {
                let x = rect.x.0 + walked_col as u16;
                let column_rect =
                    Rect::from_size((x, rect.y.0), (available_column_width as u16, 1)).unwrap();
                walked_col += available_column_width;
                let column_entt = commands.push((ColumnHeader {}, parent.clone(), column_rect));
                children.0.push(column_entt);
            }
        };

    foreach_column_compute_width_and_draw((&rect, columns), cache_column_header);
}

/// Draw a Table widget within the Rect space.
///
/// Includes drawing the box, column headers, table rows and line numbers.
///
/// It is the entrypoint function of this module.
pub fn draw_table<W>(
    stdout: &mut W,
    (rect, table, columns, vscroll, selection, winbox): (
        &Rect,
        &Table,
        &Columns,
        Option<&VerticalScroll>,
        Option<&Selection>,
        Option<&WinBox>,
    ),
) -> Option<()>
where
    W: std::io::Write,
{
    // Clone the rect as mut because we need to shrink it later to draw sub-parts of the table
    let mut rect = rect.clone();

    // Draw the Window Box borders
    if let Some(winbox) = winbox {
        super::draw_winbox(stdout, (&rect, &winbox));
        rect = rect.inner()?;
    }

    // Print the column headers
    queue!(stdout, cursor::MoveTo(rect.x.0, rect.y.0)).unwrap();
    if columns.print_header {
        draw_table_headers(stdout, (&rect, columns));
        queue!(
            stdout,
            cursor::MoveDown(1),
            cursor::MoveToColumn(rect.x.0 + 1 /*begins on one*/)
        )
        .unwrap();
        rect = rect.offset_y0(1)?;
    }

    // Print table data rows
    draw_table_rows(stdout, (&rect, table, columns, vscroll, selection));

    // Successful Draw
    Some(())
}

/// Draw the Table Column headers at the table top row with style.
///
/// The function expects the cursor is at the right position already.
fn draw_table_headers<W>(stdout: &mut W, (rect, columns): (&Rect, &Columns))
where
    W: std::io::Write,
{
    let draw_column_header =
        &mut |column: &Column, column_separator: &str, available_column_width: usize| {
            let actual_column_name =
                super::format_strip_align(&column.name, available_column_width, &column.alignment);
            queue!(
                stdout,
                style::PrintStyledContent(StyledContent::new(
                    ContentStyle::default(),
                    &column_separator,
                )),
                style::PrintStyledContent(StyledContent::new(
                    column.style.clone(),
                    &actual_column_name,
                ))
            )
            .unwrap();
        };

    foreach_column_compute_width_and_draw((&rect, columns), draw_column_header);
}

/// Draw the Table rows while there is vertical space in Rect.
///
/// The function expects that the cursor is at the right position already.
fn draw_table_rows<W>(
    stdout: &mut W,
    (rect, table, columns, vscroll, selection): (
        &Rect,
        &Table,
        &Columns,
        Option<&VerticalScroll>,
        Option<&Selection>,
    ),
) where
    W: std::io::Write,
{
    let draw_row = |_iter_idx: usize, row_idx: usize, row: &Row| {
        let row_style = selection.and_then(|selected| {
            if selected.row_index == row_idx {
                return Some(selected.style);
            }
            None
        });
        let draw_cell = |column: &Column, column_separator: &str, available_column_width: usize| {
            let (content, style) = resolve_column_content(row_idx, row, &column.value, selection);
            let actual_content =
                super::format_strip_align(&content, available_column_width, &column.alignment);
            let actual_style = row_style.or(style).unwrap_or_default();
            queue!(
                stdout,
                style::PrintStyledContent(StyledContent::new(
                    actual_style.clone(),
                    &column_separator,
                )),
                style::PrintStyledContent(StyledContent::new(
                    actual_style.clone(),
                    &actual_content,
                ))
            )
            .unwrap();
        };
        foreach_column_compute_width_and_draw((&rect, columns), draw_cell);
        queue!(
            stdout,
            cursor::MoveDown(1),
            cursor::MoveToColumn(rect.x.0 + 1 /*begins on one*/)
        )
        .unwrap();
    };

    foreach_visible_row_compute_and_draw((rect, table, vscroll), draw_row);
}

/// Traverse the table rows that are visible to the drawing space and callback the drawing function.
///
/// This range considers the number of rows, the vertical scroll and the screen height.
/// Which one is the lowest determines the last row to be drawn.
///
/// draw_callback: FnMut(iter_idx, row_idx, row)
///     iter_idx: iteration index start on zero for the first callback call and increments progressively.
///     row_idx: row index in the table row vector, starts from the vertical scroll index value.
///     row: row object
fn foreach_visible_row_compute_and_draw<F>(
    (rect, table, vscroll): (&Rect, &Table, Option<&VerticalScroll>), mut draw_callback: F,
) where
    F: FnMut(usize, usize, &Row),
{
    let begin_idx = vscroll.and_then(|v| Some(v.top_row)).unwrap_or(0);
    let after_vscroll_rows_count = table.rows.len() - begin_idx;
    let visible_rows_count = std::cmp::min(after_vscroll_rows_count, rect.height() as usize);
    let end_idx = begin_idx + visible_rows_count; // index non inclusive

    for (iter_idx, row) in table.rows[begin_idx..end_idx].iter().enumerate() {
        let row_idx = begin_idx + iter_idx;
        draw_callback(iter_idx, row_idx, row);
    }
}

/// Traverse the table columns and compute some information for the drawing function.
///
/// For each column the available column width is calculated and passed to the drawing function.
/// When there is no more room in the screen, break the drawing loop.
///
/// draw_callback: FnMut(column, column_separator, available_column_width)
fn foreach_column_compute_width_and_draw<F>(
    (rect, columns): (&Rect, &Columns), mut draw_callback: F,
) where
    F: FnMut(&Column, &str, usize),
{
    let mut column_separator = "".to_string();
    let mut walked_column_width = 0;
    for (col, column) in columns.visible.iter().enumerate() {
        let available_width = rect.width() as usize - walked_column_width;
        if available_width == 0 {
            break;
        }
        let available_width = available_width - column_separator.len();
        let available_column_width = if col == columns.visible.len() - 1 {
            // extend the last column to the remainder of screen space
            available_width
        } else {
            // otherwise use the smaller value between required and available space
            std::cmp::min(column.width as usize, available_width)
        };
        draw_callback(column, &column_separator, available_column_width);
        walked_column_width += available_column_width + column_separator.len();
        column_separator = columns.separator.to_string();
    }
}

/// Resolve the Column content base on the Column Value enum.
///
/// The value will be retrieved from the row data or generated on-the-fly based on other parameters.
fn resolve_column_content(
    row_idx: usize, row: &Row, column_value: &ColumnValue, selection: Option<&Selection>,
) -> (String, Option<ContentStyle>) {
    match column_value {
        ColumnValue::BuiltIn { builtin } => match builtin {
            ColumnBuiltIn::LineNumber { mode, style } => {
                let number =
                    resolve_column_builtin_line_number(mode, row_idx, selection).to_string();
                (number.to_string(), Some(style.clone()))
            }
        },
        ColumnValue::Data { index } => {
            let empty = "".to_string();
            let data = row.get(&index).unwrap_or(&empty).to_owned();
            (data, None)
        }
    }
}

/// Resolve the Line Number value based on the mode.
///
/// The output value begins on 1 (one).
/// The `row_idx` is expected to begin on 0 (zero).
fn resolve_column_builtin_line_number(
    mode: &LineNumberMode, row_idx: usize, selection: Option<&Selection>,
) -> usize {
    match mode {
        LineNumberMode::Normal => row_idx + 1,
        LineNumberMode::Relative => selection
            .and_then(|selected| {
                let row = (selected.row_index as i32 - row_idx as i32).abs() as usize;
                Some(if row == 0 { row_idx + 1 } else { row })
            })
            .unwrap_or(row_idx + 1),
    }
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// ////////////////////////////////////////////////////////////////////////////////////////////////
///                                                                                              ///
///                                        TESTING                                               ///
///                                                                                              ///
/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// ////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crossterm::style::Attribute;
    use itertools::Itertools;

    use crate::ui::change::ChangeColumn;
    use crate::ui::layout::HorizontalAlignment;
    use crate::ui::table::{
        resolve_line_number_column_width, Column, ColumnIndex, ColumnValue, Row,
    };

    use super::*;

    /// Get common set of table components used in the Tests
    fn table_components() -> (Table, Columns) {
        let mut row1 = Row::new();
        row1.insert(ChangeColumn::Commit as ColumnIndex, String::from("8f524ac"));
        row1.insert(ChangeColumn::Number as ColumnIndex, String::from("104508"));
        row1.insert(ChangeColumn::Owner as ColumnIndex, String::from("Auto QA"));
        let mut row2 = Row::new();
        row2.insert(ChangeColumn::Commit as ColumnIndex, String::from("18d3290"));
        row2.insert(ChangeColumn::Number as ColumnIndex, String::from("104525"));
        row2.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Joao Begin"),
        );
        row2.insert(ChangeColumn::Topic as ColumnIndex, String::from("galaxy"));
        let table = Table {
            rows: vec![row1, row2],
        };
        let columns = Columns {
            print_header: true,
            visible: vec![
                Column {
                    name: "".to_string(),
                    width: resolve_line_number_column_width(table.rows.len()),
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Right,
                    value: ColumnValue::BuiltIn {
                        builtin: ColumnBuiltIn::LineNumber {
                            mode: LineNumberMode::Normal,
                            style: ContentStyle::new(),
                        },
                    },
                },
                Column {
                    name: "commit".to_string(),
                    width: 8,
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Left,
                    value: ColumnValue::Data {
                        index: ChangeColumn::Commit as ColumnIndex,
                    },
                },
                Column {
                    name: "number".to_string(),
                    width: 8,
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Left,
                    value: ColumnValue::Data {
                        index: ChangeColumn::Number as ColumnIndex,
                    },
                },
            ],
            hidden: vec![],
            separator: '|',
        };
        (table, columns)
    }

    fn strip_ansi_escapes(output: Vec<u8>) -> Vec<String> {
        let output = String::from_utf8(output).unwrap();
        let output = output
            .split("\u{1b}[1B" /* ansi move-down 1 line */)
            .map(|row| {
                let bytes = strip_ansi_escapes::strip(row.as_bytes()).unwrap();
                String::from_utf8(bytes).unwrap()
            })
            .collect_vec();
        output
    }

    fn get_reversed_rows(output: Vec<u8>) -> Vec<String> {
        let mut selections = Vec::new();
        let output = String::from_utf8(output).unwrap();
        let output = output.split("\u{1b}[1E" /* ansi new-line */);
        for row in output {
            let mut selection = String::new();
            let mut row = row.to_owned();
            while !row.is_empty() {
                let selected = row.find("\u{1b}[7m" /* ansi reverse */).and_then(|begin| {
                    row.find("\u{1b}[0m" /* ansi normal */).and_then(|end| {
                        let (_, remainder) = row.split_at(end + 4);
                        let some = Some(row[begin + 4..end].to_string());
                        row = remainder.to_string();
                        some
                    })
                });
                if let Some(selected) = selected {
                    selection += &selected;
                } else {
                    row = "".to_string();
                }
            }
            if !selection.is_empty() {
                selections.push(selection)
            }
        }
        selections
    }

    #[test]
    /// Expect all columns are visible and last one has extended space to the screen end
    fn multiple_columns_high_width() {
        let expected = vec![
            "  |commit  |number     ",
            " 1|8f524ac |104508     ",
            " 2|18d3290 |104525     ",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 3));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the all columns are shown in a exact match space for column names
    fn multiple_columns_exact_name_space() {
        let expected = vec![
            "commit  |number", //
            "",                //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (15, 1));
        let (table, mut columns) = table_components();
        columns.visible.remove(0); // remove line-number column for this test
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect a column is shown in a exact column space, other columns are not shown
    fn multiple_columns_exact_one_column_space() {
        let expected = vec![
            "commit  ", //
            "8f524ac ", //
            "",         //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (8, 2));
        let (table, mut columns) = table_components();
        columns.visible.remove(0); // remove line-number column for this test
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the first data column only show the etc. character, for 1 width only space
    fn one_column_one_space() {
        let expected = vec![
            "…", // header
            "…", // row 1
            "…", // row 2
            "",    //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (1, 3));
        let (table, mut columns) = table_components();
        columns.visible.remove(0); // remove line-number column for this test
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the first column of linenumbers only show the etc. character, for 1 width only space
    fn one_column_one_space_line_numbers() {
        let expected = vec![
            "…", // header
            "…", // row 1
            "…", // row 2
            "",    //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (1, 3));
        let (table, mut columns) = table_components();
        columns.visible.remove(0); // remove line-number column for this test
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the second columns only show the etc. character, for 1 width only space
    fn second_column_one_space() {
        let expected = vec![
            "commit  |…", //
            "8f524ac |…", //
            "18d3290 |…", //
            "",             //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (10, 3));
        let (table, mut columns) = table_components();
        columns.visible.remove(0); // remove line-number column for this test
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the second column show the etc. character for space missed by 1 char
    fn second_column_almost_full_name_space() {
        let expected = vec![
            "commit  |numb…", //
            "",                 //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (14, 1));
        let (table, mut columns) = table_components();
        columns.visible.remove(0); // remove line-number column for this test
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect column headers are not printed when this flag is disabled
    fn print_headers_disabled() {
        let expected = vec![
            " 1|8f524ac |1045…",
            " 2|18d3290 |1045…",
            "", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (17, 2));
        let (table, mut columns) = table_components();
        columns.print_header = false;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect no breakage when there are no columns to print
    fn no_visible_columns() {
        let expected = vec!["", "", ""];
        let rect = Rect::from_size_unchecked((0, 0), (14, 3));
        let (table, _) = table_components();
        let columns = Columns::default();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect column with no name to have its length still filled up with spaces
    fn column_no_name_fill_space() {
        let expected = vec![
            "  |commit  |     |number    ",
            " 1|8f524ac |     |104508    ",
            " 2|18d3290 |     |104525    ",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (28, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            2,
            Column {
                name: "".to_string(),
                width: 5,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Time as ColumnIndex,
                },
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect column with smaller length then its name to have the columns and its name constrained
    fn column_smaller_length_than_name() {
        let expected = vec![
            "  |commit  |ow…|number      ",
            " 1|8f524ac |Au…|104508      ",
            " 2|18d3290 |Jo…|104525      ",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (28, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            2,
            Column {
                name: "owner".to_string(),
                width: 3,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Owner as ColumnIndex,
                },
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect full name when column length and name length match
    fn column_name_exact_length() {
        let expected = vec![
            "  |commit  |branch|number   ", //
            "",                             //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (28, 1));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            2,
            Column {
                name: "branch".to_string(),
                width: 6,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Branch as ColumnIndex,
                },
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect no breakage when there is no table entries to print, despite visible columns
    fn no_table_entries_to_print() {
        let expected = vec![
            "  |commit  |number     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 3));
        let (_, columns) = table_components();
        let table = Table {
            rows: Vec::default(),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect to print less table entries when space is smaller than table size
    fn more_table_entries_than_space() {
        let expected = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 2));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect to print all table entries when space more space than table entries
    fn more_space_than_table_entries() {
        let expected = vec![
            "  |commit  |number     ",
            " 1|8f524ac |104508     ",
            " 2|18d3290 |104525     ",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 5)); // note the height 5 !
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect space is printed for table entries that does not have a given column
    fn space_for_optional_column_in_table_entries() {
        let expected = vec![
            "  |commit  |topic     |numb…",
            " 1|8f524ac |          |1045…",
            " 2|18d3290 |galaxy    |1045…",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (28, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            2,
            Column {
                name: "topic".to_string(),
                width: 10,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
                value: ColumnValue::Data {
                    index: ChangeColumn::Topic as ColumnIndex,
                },
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the last column is right aligned to the end of the screen space
    fn right_alignment_last_column() {
        let expected = vec![
            "  |commit  |     number",
            " 1|8f524ac |     104508",
            " 2|18d3290 |     104525",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (table, mut columns) = table_components();
        let last_idx = columns.visible.len() - 1;
        columns.visible[last_idx].alignment = HorizontalAlignment::Right;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the last column is center aligned considering the remaining screen space
    fn center_alignment_last_column() {
        let expected = vec![
            "  |commit  |  number   ",
            " 1|8f524ac |  104508   ",
            " 2|18d3290 |  104525   ",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (table, mut columns) = table_components();
        let last_idx = columns.visible.len() - 1;
        columns.visible[last_idx].alignment = HorizontalAlignment::Center;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table entries are right-aligned and large entries are cut out with etc. symbol
    fn right_alignment_mixed_width_entries() {
        let expected = vec![
            "  |commit  |    owner|number",
            " 1|8f524ac |  Auto QA|104508",
            " 2|18d3290 |Joao Beg…|104525",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (28, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            2,
            Column {
                name: "owner".to_string(),
                width: 9,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Right,
                value: ColumnValue::Data {
                    index: ChangeColumn::Owner as ColumnIndex,
                },
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table entries are center-aligned and large entries are cut out with etc. symbol
    fn center_alignment_mixed_width_entries() {
        let expected = vec![
            "  |commit  |  owner  |number",
            " 1|8f524ac | Auto QA |104508",
            " 2|18d3290 |Joao Beg…|104525",
            "",
        ];
        let rect = Rect::from_size_unchecked((0, 0), (28, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            2,
            Column {
                name: "owner".to_string(),
                width: 9,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Center,
                value: ColumnValue::Data {
                    index: ChangeColumn::Owner as ColumnIndex,
                },
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table is drawn with the first row selected
    fn select_first_row() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            " 2|18d3290 |104525     ", //
            "",                        //
        ];
        let expected_selection = vec![
            " 1|8f524ac |104508     ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (table, columns) = table_components();
        let selection = Selection {
            row_index: 0,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection), None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the table is drawn with the second row selected
    fn select_second_row() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let expected_selection = vec![
            " 2|18d3290 |104525     ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let selection = Selection {
            row_index: 1,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection), None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the table is drawn with the last row selected
    fn select_last_row() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let expected_selection = vec![
            " 3|46a003e |104455     ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let selection = Selection {
            row_index: 2,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection), None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the scroll with 0 units causes the table to be drawn starting from the first row
    fn scroll_zero() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 0 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the scroll with one units cause the table to be drawn starting from second row
    fn scroll_one_row() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 1 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the scroll with two units causes the table to be drawn starting from third row
    fn scroll_two_row() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 2 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the scroll with all rows cause the table to not be drawn
    fn scroll_all_rows() {
        let expected_output = vec![
            "  |commit  |number     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 3 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn with the second row selected when table is scrolled down
    fn selection_plus_scroll() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let expected_selection = vec![
            " 2|18d3290 |104525     ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 3));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 1 };
        let selection = Selection {
            row_index: 1,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (
                &rect,
                &table,
                &columns,
                Some(&vscroll),
                Some(&selection),
                None,
            ),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the table is drawn with the second row selected, columns headers are not printed
    fn selection_plus_scroll_minus_print_headers() {
        let expected_output = vec![
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let expected_selection = vec![
            " 2|18d3290 |104525     ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 3));
        let (mut table, mut columns) = table_components();
        columns.print_header = false;
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 1 };
        let selection = Selection {
            row_index: 1,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (
                &rect,
                &table,
                &columns,
                Some(&vscroll),
                Some(&selection),
                None,
            ),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the table is drawn respecting scroll index and screen height and selection
    fn scroll_plus_small_height() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 2|18d3290 |104525     ", // second row
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 2));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 1 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn respecting scroll index and screen height and selection and do not print column headers
    fn scroll_plus_small_height_minus_print_columns() {
        let expected_output = vec![
            " 2|18d3290 |104525     ", // second row
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 1));
        let (mut table, mut columns) = table_components();
        columns.print_header = false;
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let vscroll = VerticalScroll { top_row: 1 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn with the second row selected while still not visible
    fn selection_outside_screen_space() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            "",                        //
        ];
        let expected_selection: Vec<&str> = vec![]; //empty cause there is no visible selection
        let rect = Rect::from_size_unchecked((0, 0), (23, 2));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let selection = Selection {
            row_index: 2,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection), None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect drawing only the built-in line numbers column on exact space screen
    fn only_line_numbers_column_exact_space() {
        let expected = vec![
            "ln", //
            " 1", //
            " 2", //
            "",   //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (2, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].name = "ln".to_string();
        // remove other columns
        while columns.visible.len() > 2 {
            columns.visible.remove(2);
        }
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect drawing only the built-in line numbers column, the column is extended to the screen end
    fn only_line_numbers_column_extends_width() {
        let expected = vec![
            "  ln", //
            "   1", //
            "   2", //
            "",     //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (4, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].name = "ln".to_string();
        // remove other columns
        while columns.visible.len() > 1 {
            columns.visible.remove(1);
        }
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect drawing only the built-in line numbers column, but the width is shrunk
    fn only_line_numbers_column_small_space() {
        let expected = vec![
            "…", //
            "1",   //
            "2",   //
            "",    //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (1, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].name = "ln".to_string();
        // remove other columns
        while columns.visible.len() > 1 {
            columns.visible.remove(1);
        }
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect line-number columns are abbreviated with etc. symbol when width is small
    fn scrolled_line_numbers_two_digits_small_space() {
        let expected_output = vec![
            " ",   //
            "9",   //
            "…", //
            "…", //
            "",    //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (1, 4));
        let (mut table, columns) = table_components();
        for _ in table.rows.len()..=12 {
            let mut row3 = Row::new();
            row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
            row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
            row3.insert(
                ChangeColumn::Owner as ColumnIndex,
                String::from("Thomas Edison"),
            );
            table.rows.push(row3);
        }
        let vscroll = VerticalScroll { top_row: 8 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), None, None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect drawing built-in line numbers column with left alignment
    fn line_numbers_column_left_aligned() {
        let expected = vec![
            "ln|commit ", //
            "1 |8f524ac", //
            "2 |18d3290", //
            "",           //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (10, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].name = "ln".to_string();
        columns.visible[0].alignment = HorizontalAlignment::Left;
        // remove other columns
        while columns.visible.len() > 2 {
            columns.visible.remove(2);
        }
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect built-in line numbers column is not drawn when configured column width is zero
    fn line_numbers_column_width_zero() {
        let expected = vec![
            "|commit  |number       ", //
            "|8f524ac |104508       ", //
            "|18d3290 |104525       ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].width = 0;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect built-in line numbers column with column width 1 (one)
    fn line_numbers_column_width_one() {
        let expected = vec![
            " |commit  |number      ", //
            "1|8f524ac |104508      ", //
            "2|18d3290 |104525      ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].width = 1;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect built-in line numbers column with column width much larger then needed
    fn line_numbers_column_width_large() {
        let expected = vec![
            "     |commit  |number  ", //
            "    1|8f524ac |104508  ", //
            "    2|18d3290 |104525  ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 4));
        let (table, mut columns) = table_components();
        columns.visible[0].width = 5;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table is drawn with relative numbers to the selected third row
    fn relative_line_numbers_selected_third_row() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 2|8f524ac |104508     ", //
            " 1|18d3290 |104525     ", //
            " 3|46a003e |104455     ", // selected
            " 1|8f524ac |104508     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            "",                        //
        ];
        let expected_selection = vec![
            " 3|46a003e |104455     ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 8));
        let (mut table, mut columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        table.rows.extend(table.rows.clone());
        match &mut columns.visible[0].value {
            ColumnValue::BuiltIn { builtin } => match builtin {
                ColumnBuiltIn::LineNumber { mode, .. } => {
                    *mode = LineNumberMode::Relative;
                } // _ => {}
            },
            _ => {}
        }
        let selection = Selection {
            row_index: 2,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection), None),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the table is drawn with normal numbers when it is relative but there is no selection
    fn relative_line_numbers_no_selection() {
        let expected_output = vec![
            "  |commit  |number     ", //
            " 1|8f524ac |104508     ", //
            " 2|18d3290 |104525     ", //
            " 3|46a003e |104455     ", //
            " 4|8f524ac |104508     ", //
            " 5|18d3290 |104525     ", //
            " 6|46a003e |104455     ", //
            "",                        //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 8));
        let (mut table, mut columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        table.rows.extend(table.rows.clone());
        match &mut columns.visible[0].value {
            ColumnValue::BuiltIn { builtin } => match builtin {
                ColumnBuiltIn::LineNumber { mode, .. } => {
                    *mode = LineNumberMode::Relative;
                } // _ => {}
            },
            _ => {}
        }
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None, None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn with normal numbers column with 3 digits
    fn normal_line_numbers_3_digit_table_row() {
        let expected_output = vec![
            "   |commit  |number    ", //
            " 98|46a0100 |104552    ", //
            " 99|46a0102 |104553    ", //
            "100|46a0104 |104554    ", // selected
            "101|46a0106 |104555    ", //
            "102|46a0108 |104556    ", //
            "103|46a010a |104557    ", //
            "",                        //
        ];
        let expected_selection = vec![
            "100|46a0104 |104554    ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 7));
        let (mut table, mut columns) = table_components();
        for idx in table.rows.len()..=102 {
            let mut row = Row::new();
            let commit = 74055742 + (idx * 2);
            let number = 104455 + idx;
            row.insert(ChangeColumn::Commit as ColumnIndex, format!("{:x}", commit));
            row.insert(ChangeColumn::Number as ColumnIndex, format!("{}", number));
            table.rows.push(row);
        }
        columns.visible[0].width = resolve_line_number_column_width(table.rows.len());
        let vscroll = VerticalScroll { top_row: 97 };
        let selection = Selection {
            row_index: 99,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (
                &rect,
                &table,
                &columns,
                Some(&vscroll),
                Some(&selection),
                None,
            ),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the table is drawn with relative numbers column with 3 digits
    fn relative_line_numbers_3_digit_table_row() {
        let expected_output = vec![
            "   |commit  |number    ", //
            "  2|46a0100 |104552    ", //
            "  1|46a0102 |104553    ", //
            "100|46a0104 |104554    ", // selected
            "  1|46a0106 |104555    ", //
            "  2|46a0108 |104556    ", //
            "  3|46a010a |104557    ", //
            "",                        //
        ];
        let expected_selection = vec![
            "100|46a0104 |104554    ", //
        ];
        let rect = Rect::from_size_unchecked((0, 0), (23, 7));
        let (mut table, mut columns) = table_components();
        for idx in table.rows.len()..=102 {
            let mut row = Row::new();
            let commit = 74055742 + (idx * 2);
            let number = 104455 + idx;
            row.insert(ChangeColumn::Commit as ColumnIndex, format!("{:x}", commit));
            row.insert(ChangeColumn::Number as ColumnIndex, format!("{}", number));
            table.rows.push(row);
        }
        columns.visible[0].width = resolve_line_number_column_width(table.rows.len());
        match &mut columns.visible[0].value {
            ColumnValue::BuiltIn { builtin } => match builtin {
                ColumnBuiltIn::LineNumber { mode, .. } => {
                    *mode = LineNumberMode::Relative;
                } // _ => {}
            },
            _ => {}
        }
        let vscroll = VerticalScroll { top_row: 97 };
        let selection = Selection {
            row_index: 99,
            style: ContentStyle::new().attribute(Attribute::Reverse),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (
                &rect,
                &table,
                &columns,
                Some(&vscroll),
                Some(&selection),
                None,
            ),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }
}
