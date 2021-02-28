use crate::ui::r#box::{Box, Rect};
use crate::ui::scroll::Scroll;
use crossterm::style::ContentStyle;
use crossterm::{cursor, queue, style};

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// Column
/// ////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Column<'a> {
    pub name: &'a str,
    pub width: u16,
    pub style: ContentStyle,
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// List
/// ////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct List<'a> {
    pub print_column_headers: bool, // TODO: implement
    pub columns: &'a [Column<'a>],
    pub data: &'a [&'a [&'a str]],
}

pub fn draw_scrollable_list<W>(stdout: &mut W, list: &List, scroll: &Scroll, rect: &Rect)
where
    W: std::io::Write,
    // D: ExactSizeIterator<Item = I>,
    // I: IntoIterator<Item = S>,
    // S: AsRef<str>,
{
    let mut walked_len = 0;
    for (_col, column) in list.columns.iter().enumerate() {
        let remaining_len = {
            let value = (rect.width() - walked_len) as i32;
            if value.is_positive() {
                value as u16
            } else {
                0 as u16
            }
        };

        // HEADER
        let column_len = std::cmp::min(column.width, remaining_len);
        let column_name = column
            .name
            .split_at(std::cmp::min(column_len as usize, column.name.len()))
            .0;
        queue!(
            stdout,
            cursor::MoveTo(rect.x.0 + walked_len, rect.y.0),
            style::PrintStyledContent(style::StyledContent::new(column.style, column_name))
        )
        .unwrap();

        walked_len += column_len;
    }

    // DATA
    let offset_row = scroll.top_row as usize;
    let max_rows = std::cmp::min(
        list.data.len() - offset_row,
        (rect.height() - /*header*/1) as usize,
    );

    for (row, line) in list.data.into_iter().enumerate() {
        if row >= max_rows {
            break;
        }
        let mut walked_len = 0;
        for (col, value) in line.into_iter().enumerate() {
            if col >= list.columns.len() {
                break;
            }
            let column = &list.columns[col];
            let remaining_len = {
                let value = (rect.width() - walked_len) as i32;
                if value.is_positive() {
                    value as u16
                } else {
                    0 as u16
                }
            };
            let column_len = std::cmp::min(column.width, remaining_len);
            let value = value
                .split_at(std::cmp::min(column_len as usize, value.len()))
                .0;
            queue!(
                stdout,
                cursor::MoveTo(rect.x.0 + walked_len, rect.y.0 + row as u16 + 1),
                style::Print(value)
            )
            .unwrap();

            walked_len += column_len;
        }
    }
}

pub fn resize_scrollable_list_box(
    list: &mut List, scroll: &mut Scroll, r#box: &mut Box, cols: u16, rows: u16,
) {
    let scroll_diff = (r#box.rect.height() as i32) - (rows as i32);
    let rows_after_scroll = (list.data.len() - scroll.top_row as usize) as i32;
    if (rows_after_scroll < (rows - 3) as i32) && scroll_diff.is_negative() {
        crate::ui::scroll::scroll_list(scroll, list, &r#box.rect, scroll_diff);
    }
    r#box.rect = Rect::from_size((r#box.rect.x.0, r#box.rect.y.0), (cols, rows));
}

// FUNCTIONAL 1
//
// pub fn draw_scrollable_list<'a, W, D, I, S>(
//     stdout: &mut W, list: &List, scroll: &Scroll, rect: &Rect, data: D,
// ) where
//     W: std::io::Write,
//     D: ExactSizeIterator<Item = I>,
//     I: IntoIterator<Item = S>,
//     S: AsRef<str>,
// {
//     let mut walked_len = 0;
//     for (col, column) in list.columns.iter().enumerate() {
//         let remaining_len = {
//             let value = (rect.width() - walked_len) as i32;
//             if value.is_positive() {
//                 value as u16
//             } else {
//                 0 as u16
//             }
//         };
//
//         // HEADER
//         let column_len = std::cmp::min(column.width, remaining_len);
//         let column_name = column
//             .name
//             .split_at(std::cmp::min(column_len as usize, column.name.len()))
//             .0;
//         queue!(
//             stdout,
//             cursor::MoveTo(rect.x.0 + walked_len, rect.y.0),
//             style::PrintStyledContent(style::StyledContent::new(column.style, column_name))
//         )
//             .unwrap();
//
//         walked_len += column_len;
//     }
//
//     // DATA
//     let offset_row = scroll.top_row as usize;
//     let max_rows = std::cmp::min(
//         data.len() - offset_row,
//         (rect.height() - /*header*/1) as usize,
//     );
//
//     for (row, line) in data.enumerate() {
//         if row >= max_rows {
//             break;
//         }
//         let mut walked_len = 0;
//         for (col, value) in line.into_iter().enumerate() {
//             let value = value.as_ref();
//             if col >= list.columns.len() {
//                 break;
//             }
//             let column = list.columns.index(col);
//             let remaining_len = {
//                 let value = (rect.width() - walked_len) as i32;
//                 if value.is_positive() {
//                     value as u16
//                 } else {
//                     0 as u16
//                 }
//             };
//             let column_len = std::cmp::min(column.width, remaining_len);
//             let value = value
//                 .split_at(std::cmp::min(column_len as usize, value.len()))
//                 .0;
//             queue!(
//                 stdout,
//                 cursor::MoveTo(rect.x.0 + walked_len, rect.y.0 + row as u16 + 1),
//                 style::Print(value)
//             )
//                 .unwrap();
//
//             walked_len += column_len;
//         }
//     }
// }
