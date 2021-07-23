use chrono::{DateTime, TimeZone, Utc};

pub mod validate;

/// Function to check if boolean is false.
/// Used for serde 'path' attributes.
pub fn is_false(b: &bool) -> bool {
  !*b
}

/// Dynamic short format for DataTime
pub fn format_short_datetime(from_utc: &DateTime<Utc>) -> String {
  use chrono::format::{Fixed, Item, Numeric, Pad};
  use chrono::offset::Local;
  use chrono::Datelike;

  let from_local = Local.from_utc_datetime(&from_utc.naive_utc());
  let now_local = Local::now();
  let duration = now_local - from_local;

  let mut format_items = Vec::new();
  if (duration.num_days() == 0) && (from_local.day() == now_local.day()) {
    format_items.reserve(5);
    format_items.push(Item::Numeric(Numeric::Hour12, Pad::Zero));
    format_items.push(Item::Literal(":"));
    format_items.push(Item::Numeric(Numeric::Minute, Pad::Zero));
    format_items.push(Item::Literal(" "));
    format_items.push(Item::Fixed(Fixed::UpperAmPm));
  } else {
    format_items.reserve(5);
    format_items.push(Item::Fixed(Fixed::ShortMonthName));
    format_items.push(Item::Literal(" "));
    format_items.push(Item::Numeric(Numeric::Day, Pad::Zero));
    if from_local.year() != now_local.year() {
      format_items.push(Item::Literal(", "));
      format_items.push(Item::Numeric(Numeric::Year, Pad::Zero));
    }
  }

  from_local.format_with_items(format_items.into_iter()).to_string()
}

/// Dynamic long format for DataTime
pub fn format_long_datetime(from_utc: &DateTime<Utc>) -> String {
  use chrono::format::{Fixed, Item, Numeric, Pad};
  use chrono::offset::Local;

  let mut format_items = Vec::new();
  format_items.reserve(5);
  format_items.push(Item::Fixed(Fixed::ShortWeekdayName));
  format_items.push(Item::Literal(" "));
  format_items.push(Item::Fixed(Fixed::ShortMonthName));
  format_items.push(Item::Literal(" "));
  format_items.push(Item::Numeric(Numeric::Day, Pad::None));
  format_items.push(Item::Literal(" "));
  format_items.push(Item::Numeric(Numeric::Hour, Pad::Zero));
  format_items.push(Item::Literal(":"));
  format_items.push(Item::Numeric(Numeric::Minute, Pad::Zero));
  format_items.push(Item::Literal(":"));
  format_items.push(Item::Numeric(Numeric::Second, Pad::Zero));
  format_items.push(Item::Literal(" "));
  format_items.push(Item::Numeric(Numeric::Year, Pad::None));
  format_items.push(Item::Literal(" "));
  format_items.push(Item::Fixed(Fixed::TimezoneOffset));

  let from_local = Local.from_utc_datetime(&from_utc.naive_utc());

  from_local.format_with_items(format_items.into_iter()).to_string()
}
