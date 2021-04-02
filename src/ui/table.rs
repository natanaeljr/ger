use crate::ui::layout::{HorizontalAlignment, LineNumberMode};
use crate::ui::term::TermUSize;
use crossterm::style::ContentStyle;
use std::collections::HashMap;

/// Column Index size.
/// Defined because it is common in multiple places.
pub type ColumnIndex = u8;

/// A Table Row type.
/// Maps columns by its index to the column content of this row.
/// The HashMap allows for columns to be selected dynamically instead of being linked to
/// contiguous array indices, thus the displayed columns can be "cherry-picked".
pub type Row = HashMap<ColumnIndex, String>;

/// A Table is a widget component for displaying data as a spreadsheet.
#[derive(Default, Debug, Clone)]
pub struct Table {
    pub rows: Vec<Row>,
}

/// Columns is a component related to the Table component.
/// Defines which columns are available to be displayed in the Table.
#[derive(Default, Debug, Clone)]
pub struct Columns {
    pub print_header: bool,
    pub visible: Vec<Column>,
    pub hidden: Vec<Column>,
    pub separator: char,
}

/// Column's information.
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub width: TermUSize,
    pub style: ContentStyle,
    pub alignment: HorizontalAlignment,
    pub value: ColumnValue,
}

#[derive(Debug, Clone)]
pub enum ColumnValue {
    /// Built-in columns are generated when drawing.
    BuiltIn { builtin: ColumnBuiltIn },
    /// Data columns references the data in the Table's row array.
    Data { index: ColumnIndex },
}

/// List of available built-in columns for tables.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColumnBuiltIn {
    LineNumber {
        mode: LineNumberMode,
        style: ContentStyle,
    },
}

/// VerticalScroll is a component that controls the visible rows of lists and tables.
/// The top_row indicates which row index is the first printed on the screen.
#[derive(Default, Debug, Clone)]
pub struct VerticalScroll {
    pub top_row: usize,
}

/// Selection is a component that indicates which row in a list/table is selected.
#[derive(Default, Debug, Clone)]
pub struct Selection {
    pub row_index: usize,
    pub style: ContentStyle,
}

/// Resolve what the width for the line number column should be.
///
/// That is resolved based on the total number of rows that this table can have.
/// The resulting width is the number of digits for the maximum row.
/// Otherwise the default width is always 2.
pub fn resolve_line_number_column_width(rows_len: usize) -> TermUSize {
    // default line number width is two digits wide
    if rows_len > 99 {
        rows_len.to_string().len() as TermUSize
    } else {
        2
    }
}
