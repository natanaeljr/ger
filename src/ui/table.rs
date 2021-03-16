use crate::ui::layout::HorizontalAlignment;
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
#[derive(Debug, Clone)]
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

/// A Column's information.
#[derive(Debug, Clone)]
pub struct Column {
    pub index: ColumnIndex,
    pub name: String,
    pub width: TermUSize,
    pub style: ContentStyle,
    pub alignment: HorizontalAlignment,
}
