use crate::ui::layout::{HorizontalAlignment, LineNumberMode};
use crate::ui::r#box::Rect;
use crate::ui::table::{
    Column, ColumnBuiltIn, ColumnValue, Columns, Row, Selection, Table, VerticalScroll,
};
use crossterm::style::{ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};

/// Draw a Table widget within the Rect space.
///
/// This is the entrypoint function of this module.
/// Includes drawing the Column headers, and line numbers.
pub fn draw_table<W>(
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
    // Clone the rect because we need to shrink it later to draw sub-parts of the table
    let mut rect = rect.clone();
    // Reposition the cursor to the top of our widget space
    queue!(stdout, cursor::MoveTo(rect.x.0, rect.y.0)).unwrap();
    // Print the column headers
    if columns.print_header {
        draw_table_headers(stdout, (&rect, columns));
        queue!(stdout, cursor::MoveToNextLine(1)).unwrap();
        rect.y.0 += 1;
    }
    // Check if, after the rect has been modified, it still has a valid space to continue drawing
    if rect.valid() {
        draw_table_rows(stdout, (&rect, table, columns, vscroll, selection));
    }
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
                formatted_column_content(&column.name, &column.alignment, available_column_width);
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
                formatted_column_content(&content, &column.alignment, available_column_width);
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
        queue!(stdout, cursor::MoveToNextLine(1)).unwrap();
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

/// Format the string according to the available printable space.
///
/// Example:
/// content: "abc", available_column_width: 5, output: "abc  "
/// content: "abc", available_column_width: 3, output: "abc"
/// content: "abcde", available_column_width: 3, output: "ab…"
fn formatted_column_content(
    content: &String, alignment: &HorizontalAlignment, available_column_width: usize,
) -> String {
    if available_column_width == 0 {
        Default::default()
    } else if content.len() > available_column_width {
        // Cut the string and append a etc. symbol to the end
        let (content_split, _) = content.split_at(available_column_width - 1);
        let content = format!("{}…", content_split);
        format_aligned(&content, alignment, available_column_width - 1)
    } else {
        // Fill blank with space character
        format_aligned(content, alignment, available_column_width)
    }
}

/// Format content with desired horizontal alignment.
///
/// Additional space (the character) is added to fill up the width.
fn format_aligned(content: &String, alignment: &HorizontalAlignment, width: usize) -> String {
    match alignment {
        HorizontalAlignment::Left => format!("{: <1$}", content, width),
        HorizontalAlignment::Center => format!("{: ^1$}", content, width),
        HorizontalAlignment::Right => format!("{: >1$}", content, width),
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
    use super::*;
    use crate::ui::change::ChangeColumn;
    use crate::ui::layout::HorizontalAlignment;
    use crate::ui::table::{Column, ColumnIndex, ColumnValue, Row};
    use itertools::Itertools;

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
            .split("\u{1b}[1E" /* ansi new-line */)
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
            "commit  |number     ",
            "8f524ac |104508     ",
            "18d3290 |104525     ",
            "",
        ];
        let rect = Rect::from_size((0, 0), (20, 3));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
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
        let rect = Rect::from_size((0, 0), (15, 1));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
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
        let rect = Rect::from_size((0, 0), (8, 2));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the first column only show the etc. character, for 1 width only space
    fn one_column_one_space() {
        let expected = vec![
            "…", // header
            "…", // row 1
            "…", // row 2
            "",    //
        ];
        let rect = Rect::from_size((0, 0), (1, 3));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
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
        let rect = Rect::from_size((0, 0), (10, 3));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
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
        let rect = Rect::from_size((0, 0), (14, 1));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect columns are not printed when this flag is disabled
    fn print_headers_disabled() {
        let expected = vec![
            "8f524ac |1045…",
            "18d3290 |1045…",
            "", //
        ];
        let rect = Rect::from_size((0, 0), (14, 2));
        let (table, mut columns) = table_components();
        columns.print_header = false;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect no breakage when there are no columns to print
    fn no_visible_columns() {
        let expected = vec!["", "", ""];
        let rect = Rect::from_size((0, 0), (14, 3));
        let (table, _) = table_components();
        let columns = Columns::default();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect column with no name to have its length still filled up with spaces
    fn column_no_name_fill_space() {
        let expected = vec![
            "commit  |     |number    ",
            "8f524ac |     |104508    ",
            "18d3290 |     |104525    ",
            "",
        ];
        let rect = Rect::from_size((0, 0), (25, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            1,
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
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect column with smaller length then its name to have the columns and its name constrained
    fn column_smaller_length_than_name() {
        let expected = vec![
            "commit  |ow…|number      ",
            "8f524ac |Au…|104508      ",
            "18d3290 |Jo…|104525      ",
            "",
        ];
        let rect = Rect::from_size((0, 0), (25, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            1,
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
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect full name when column length and name length match
    fn column_name_exact_length() {
        let expected = vec![
            "commit  |branch|number   ", //
            "",                          //
        ];
        let rect = Rect::from_size((0, 0), (25, 1));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            1,
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
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect no breakage when there is no table entries to print, despite visible columns
    fn no_table_entries_to_print() {
        let expected = vec![
            "commit  |number     ", //
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 3));
        let (_, columns) = table_components();
        let table = Table {
            rows: Vec::default(),
        };
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect to print less table entries when space is smaller than table size
    fn more_table_entries_than_space() {
        let expected = vec![
            "commit  |number     ", //
            "8f524ac |104508     ", //
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 2));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect to print all table entries when space more space than table entries
    fn more_space_than_table_entries() {
        let expected = vec![
            "commit  |number     ",
            "8f524ac |104508     ",
            "18d3290 |104525     ",
            "",
        ];
        let rect = Rect::from_size((0, 0), (20, 5)); // note the height 5 !
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect space is printed for table entries that does not have a given column
    fn space_for_optional_column_in_table_entries() {
        let expected = vec![
            "commit  |topic     |numb…",
            "8f524ac |          |1045…",
            "18d3290 |galaxy    |1045…",
            "",
        ];
        let rect = Rect::from_size((0, 0), (25, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            1,
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
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the last column is right aligned to the end of the screen space
    fn right_alignment_last_column() {
        let expected = vec![
            "commit  |     number",
            "8f524ac |     104508",
            "18d3290 |     104525",
            "",
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
        let (table, mut columns) = table_components();
        columns.visible[1].alignment = HorizontalAlignment::Right;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the last column is center aligned considering the remaining screen space
    fn center_alignment_last_column() {
        let expected = vec![
            "commit  |  number   ",
            "8f524ac |  104508   ",
            "18d3290 |  104525   ",
            "",
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
        let (table, mut columns) = table_components();
        columns.visible[1].alignment = HorizontalAlignment::Center;
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table entries are right-aligned and large entries are cut out with etc. symbol
    fn right_alignment_mixed_width_entries() {
        let expected = vec![
            "commit  |    owner|number",
            "8f524ac |  Auto QA|104508",
            "18d3290 |Joao Beg…|104525",
            "",
        ];
        let rect = Rect::from_size((0, 0), (25, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            1,
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
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table entries are center-aligned and large entries are cut out with etc. symbol
    fn center_alignment_mixed_width_entries() {
        let expected = vec![
            "commit  |  owner  |number",
            "8f524ac | Auto QA |104508",
            "18d3290 |Joao Beg…|104525",
            "",
        ];
        let rect = Rect::from_size((0, 0), (25, 3));
        let (table, mut columns) = table_components();
        columns.visible.insert(
            1,
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
        draw_table(&mut output, (&rect, &table, &columns, None, None));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Expect the table is drawn with the first row selected
    fn select_first_row() {
        let expected_output = vec![
            "commit  |number     ", //
            "8f524ac |104508     ", //
            "18d3290 |104525     ", //
            "",                     //
        ];
        let expected_selection = vec![
            "8f524ac |104508     ", //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
        let (table, columns) = table_components();
        let selection = Selection { row_index: 0 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection)),
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
            "commit  |number     ", //
            "8f524ac |104508     ", //
            "18d3290 |104525     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let expected_selection = vec![
            "18d3290 |104525     ", //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let selection = Selection { row_index: 1 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection)),
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
            "commit  |number     ", //
            "8f524ac |104508     ", //
            "18d3290 |104525     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let expected_selection = vec![
            "46a003e |104455     ", //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let selection = Selection { row_index: 2 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection)),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }

    #[test]
    /// Expect the scroll with 0 units cause the table to be drawn starting from the first row
    fn scroll_zero() {
        let expected_output = vec![
            "commit  |number     ", //
            "8f524ac |104508     ", //
            "18d3290 |104525     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
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
        draw_table(&mut output, (&rect, &table, &columns, Some(&vscroll), None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the scroll with one units cause the table to be drawn starting from row 1
    fn scroll_one_row() {
        let expected_output = vec![
            "commit  |number     ", //
            "18d3290 |104525     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
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
        draw_table(&mut output, (&rect, &table, &columns, Some(&vscroll), None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the scroll with two units cause the table to be drawn starting from row 2
    fn scroll_two_row() {
        let expected_output = vec![
            "commit  |number     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
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
        draw_table(&mut output, (&rect, &table, &columns, Some(&vscroll), None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the scroll with all rows cause the table to not be drawn
    fn scroll_all_rows() {
        let expected_output = vec![
            "commit  |number     ", //
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 4));
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
        draw_table(&mut output, (&rect, &table, &columns, Some(&vscroll), None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn with the second row selected
    fn selection_plus_scroll() {
        let expected_output = vec![
            "commit  |number     ", //
            "18d3290 |104525     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let expected_selection = vec![
            "18d3290 |104525     ", //
        ];
        let rect = Rect::from_size((0, 0), (20, 3));
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
        let selection = Selection { row_index: 1 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), Some(&selection)),
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
            "18d3290 |104525     ", //
            "46a003e |104455     ", //
            "",                     //
        ];
        let expected_selection = vec![
            "18d3290 |104525     ", //
        ];
        let rect = Rect::from_size((0, 0), (20, 3));
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
        let selection = Selection { row_index: 1 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, Some(&vscroll), Some(&selection)),
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
            "commit  |number     ", //
            "18d3290 |104525     ", // second row
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 2));
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
        draw_table(&mut output, (&rect, &table, &columns, Some(&vscroll), None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn respecting scroll index and screen height and selection and do not print column headers
    fn scroll_plus_small_height_minus_print_columns() {
        let expected_output = vec![
            "18d3290 |104525     ", // second row
            "",                     //
        ];
        let rect = Rect::from_size((0, 0), (20, 1));
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
        draw_table(&mut output, (&rect, &table, &columns, Some(&vscroll), None));
        let actual_output = strip_ansi_escapes(output.clone());
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    /// Expect the table is drawn with the second row selected
    fn selection_outside_screen_space() {
        let expected_output = vec![
            "commit  |number     ", //
            "8f524ac |104508     ", //
            "",                     //
        ];
        let expected_selection: Vec<&str> = vec![]; //empty cause there is no visible selection
        let rect = Rect::from_size((0, 0), (20, 2));
        let (mut table, columns) = table_components();
        let mut row3 = Row::new();
        row3.insert(ChangeColumn::Commit as ColumnIndex, String::from("46a003e"));
        row3.insert(ChangeColumn::Number as ColumnIndex, String::from("104455"));
        row3.insert(
            ChangeColumn::Owner as ColumnIndex,
            String::from("Thomas Edison"),
        );
        table.rows.push(row3);
        let selection = Selection { row_index: 2 };
        let mut output: Vec<u8> = Vec::new();
        draw_table(
            &mut output,
            (&rect, &table, &columns, None, Some(&selection)),
        );
        let actual_output = strip_ansi_escapes(output.clone());
        let actual_selection = get_reversed_rows(output.clone());
        assert_eq!(expected_output, actual_output);
        assert_eq!(expected_selection, actual_selection);
    }
}
