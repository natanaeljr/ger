use crate::ui::layout::HorizontalAlignment;
use crate::ui::r#box::Rect;
use crate::ui::table::{Column, Columns, Table};
use crossterm::style::{Attribute, ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};

/// Draw a Table widget within the Rect space.
///
/// Includes drawing the Column headers, and line numbers.
pub fn draw_table<W>(stdout: &mut W, (rect, table, columns): (&Rect, &Table, &Columns))
where
    W: std::io::Write,
{
    let mut rect = rect.clone();

    queue!(stdout, cursor::MoveTo(rect.x.0, rect.y.0)).unwrap();

    if columns.print_header {
        draw_table_headers(stdout, (&rect, columns));
        queue!(stdout, cursor::MoveToNextLine(1)).unwrap();
        rect.y.0 += 1;
    }

    if rect.valid() {
        draw_table_rows(stdout, (&rect, table, columns));
    }
}

/// Draw the Table Column headers at the table top row.
fn draw_table_headers<W>(stdout: &mut W, (rect, columns): (&Rect, &Columns))
where
    W: std::io::Write,
{
    let mut column_separator_style = ContentStyle::default();
    let draw_column_header =
        &mut |column: &Column, column_separator: &str, available_column_width: usize| {
            let actual_column_name =
                formatted_column_content(&column.name, &column.alignment, available_column_width);
            queue!(
                stdout,
                style::PrintStyledContent(StyledContent::new(
                    column_separator_style.clone(),
                    &column_separator,
                )),
                style::PrintStyledContent(StyledContent::new(
                    column.style.clone(),
                    &actual_column_name,
                ))
            )
            .unwrap();
            column_separator_style = column.style.clone();
        };

    foreach_column_compute_width_and_draw((rect, columns), draw_column_header);
}

/// Draw the Table rows while there is vertical space in Rect.
fn draw_table_rows<W>(stdout: &mut W, (rect, table, columns): (&Rect, &Table, &Columns))
where
    W: std::io::Write,
{
    for (idx, row) in table.rows.iter().enumerate() {
        if idx == rect.height() as usize {
            break;
        }
        let draw_cell_content =
            &mut |column: &Column, column_separator: &str, available_column_width: usize| {
                let empty = "".to_string();
                let content = row.get(&column.index).unwrap_or(&empty);
                let actual_content =
                    formatted_column_content(content, &column.alignment, available_column_width);
                queue!(
                    stdout,
                    style::PrintStyledContent(StyledContent::new(
                        ContentStyle::new(),
                        &column_separator,
                    )),
                    style::PrintStyledContent(StyledContent::new(
                        ContentStyle::new(),
                        &actual_content,
                    ))
                )
                .unwrap()
            };
        foreach_column_compute_width_and_draw((rect, columns), draw_cell_content);
        queue!(stdout, cursor::MoveToNextLine(1)).unwrap();
    }
}

/// Traverse the table columns and compute some information for the drawing function.
///
/// For each column the available column width is calculated and passed to the drawing function.
/// When there is not more room in the screen, break the drawing loop.
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
fn format_aligned(content: &String, alignment: &HorizontalAlignment, width: usize) -> String {
    match alignment {
        HorizontalAlignment::Left => format!("{: <1$}", content, width),
        HorizontalAlignment::Center => format!("{: ^1$}", content, width),
        HorizontalAlignment::Right => format!("{: >1$}", content, width),
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
    use crate::ui::table::{Column, ColumnIndex, Row};
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
                    index: ChangeColumn::Commit as ColumnIndex,
                    name: "commit".to_string(),
                    width: 8,
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Left,
                },
                Column {
                    index: ChangeColumn::Number as ColumnIndex,
                    name: "number".to_string(),
                    width: 8,
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Left,
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
            .split("\u{1b}[1E") // ansi new-line
            .map(|row| {
                let bytes = strip_ansi_escapes::strip(row.as_bytes()).unwrap();
                String::from_utf8(bytes).unwrap()
            })
            .collect_vec();
        output
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
                index: ChangeColumn::Time as ColumnIndex,
                name: "".to_string(),
                width: 5,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
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
                index: ChangeColumn::Owner as ColumnIndex,
                name: "owner".to_string(),
                width: 3,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
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
                index: ChangeColumn::Branch as ColumnIndex,
                name: "branch".to_string(),
                width: 6,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
                index: ChangeColumn::Topic as ColumnIndex,
                name: "topic".to_string(),
                width: 10,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Left,
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
        draw_table(&mut output, (&rect, &table, &columns));
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
                index: ChangeColumn::Owner as ColumnIndex,
                name: "owner".to_string(),
                width: 9,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Right,
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
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
                index: ChangeColumn::Owner as ColumnIndex,
                name: "owner".to_string(),
                width: 9,
                style: ContentStyle::new(),
                alignment: HorizontalAlignment::Center,
            },
        );
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
        let output = strip_ansi_escapes(output);
        assert_eq!(expected, output);
    }
}
