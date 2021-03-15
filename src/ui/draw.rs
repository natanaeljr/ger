use crate::ui::r#box::Rect;
use crate::ui::table::{Columns, Table};
use crossterm::style::{ContentStyle, StyledContent};
use crossterm::{cursor, queue, style};

/// Draw a Table widget within the Rect space.
///
/// Includes drawing the Column headers, and line numbers.
pub fn draw_table<W>(stdout: &mut W, (rect, _table, columns): (&Rect, &Table, &Columns))
where
    W: std::io::Write,
{
    queue!(stdout, cursor::MoveTo(rect.x.0, rect.y.0)).unwrap();

    if columns.print_header {
        draw_table_header(stdout, (rect, columns));
    }
}

/// Draw the Table Column headers in the first table row.
fn draw_table_header<W>(stdout: &mut W, (rect, columns): (&Rect, &Columns))
where
    W: std::io::Write,
{
    let mut column_separator = StyledContent::new(ContentStyle::default(), "");
    let mut walked_column_len = 0;
    for (col, column) in columns.visible.iter().enumerate() {
        let available_width = rect.width() as usize - walked_column_len;
        if available_width == 0 {
            break;
        }
        let available_width = available_width - column_separator.content().len();
        let available_column_len = if col < columns.visible.len() - 1 {
            std::cmp::min(column.len as usize, available_width)
        } else {
            // extend the last column to the remainder of screen space
            available_width
        };
        let actual_column_name = formatted_column_content(&column.name, available_column_len);
        queue!(
            stdout,
            style::PrintStyledContent(column_separator.clone()),
            style::PrintStyledContent(StyledContent::new(
                column.style.clone(),
                &actual_column_name,
            ))
        )
        .unwrap();
        walked_column_len += available_column_len + column_separator.content().len();
        column_separator = StyledContent::new(column.style.clone(), "|"); // must be one character!
    }
}

/// Format the string according to the available printable space.
///
/// Example:
/// content: "abc", available_column_len: 5, output: "abc  "
/// content: "abc", available_column_len: 3, output: "abc"
/// content: "abcde", available_column_len: 3, output: "ab…"
fn formatted_column_content(content: &String, available_column_len: usize) -> String {
    if available_column_len == 0 {
        String::default()
    } else if content.len() > available_column_len {
        // Cut the string and append a etc. symbol to the end
        let mut new = content.split_at(available_column_len - 1).0.to_owned();
        new.push('…');
        new
    } else if content.len() < available_column_len {
        // Fill blank with space character
        let fill = " ".repeat(available_column_len - content.len());
        let mut new = content.to_owned();
        new.push_str(&fill);
        new
    } else {
        // Exact size string
        content.to_owned()
    }
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// TESTS
/// ////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::*;
    use crate::ui::change::ChangeColumn;
    use crate::ui::layout::HorizontalAlignment;
    use crate::ui::table::{Column, ColumnIndex, Row};

    // TODO:
    // - no visible columns
    // - different column sizes
    // - empty column name
    // - column name larger than column size
    // - 1x1 rect space (test w/ all other combinations)
    // - no printable column headers

    fn table_components() -> (Table, Columns) {
        let mut row = Row::new();
        row.insert(ChangeColumn::Commit as ColumnIndex, String::from("8f524ac"));
        row.insert(ChangeColumn::Number as ColumnIndex, String::from("104508"));
        row.insert(ChangeColumn::Owner as ColumnIndex, String::from("Auto QA"));
        let table = Table { data: vec![row] };
        let columns = Columns {
            print_header: true,
            visible: vec![
                Column {
                    index: ChangeColumn::Commit as ColumnIndex,
                    name: "commit".to_string(),
                    len: 8,
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Left,
                },
                Column {
                    index: ChangeColumn::Number as ColumnIndex,
                    name: "number".to_string(),
                    len: 8,
                    style: ContentStyle::new(),
                    alignment: HorizontalAlignment::Left,
                },
            ],
            hidden: vec![],
        };
        (table, columns)
    }

    #[test]
    fn first_test() {
        let rect = Rect::from_size((0, 0), (20, 1));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
        let output = strip_ansi_escapes::strip(&output).unwrap();
        let expected = "commit  |number     ";
        assert_eq!(expected, String::from_utf8(output).unwrap())
    }

    #[test]
    fn second_test() {
        let rect = Rect::from_size((0, 0), (8, 1));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
        let output = strip_ansi_escapes::strip(&output).unwrap();
        let expected = "commit  ";
        assert_eq!(expected, String::from_utf8(output).unwrap())
    }

    #[test]
    fn third_test() {
        let rect = Rect::from_size((0, 0), (10, 1));
        let (table, columns) = table_components();
        let mut output: Vec<u8> = Vec::new();
        draw_table(&mut output, (&rect, &table, &columns));
        let output = strip_ansi_escapes::strip(&output).unwrap();
        let expected = "commit  |…";
        assert_eq!(expected, String::from_utf8(output).unwrap())
    }
}
