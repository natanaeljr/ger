use num_traits::cast::ToPrimitive;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Rect represents a rectangle on a terminal grid of columns and rows.
///
/// A valid rectangle as at least one column and one row.
///
/// Model is top to bottom, left to right.
/// (x.0,y.0)    (x.1,y.0)
///     +-----------+
///     |           |
///     |           |
///     +-----------+
/// (x.0,y.1)    (x.1,y.1)
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Copy, Clone)]
pub struct Rect {
  /// X (begin, end)
  pub x: (u16, u16),
  /// Y (begin, end)
  pub y: (u16, u16),
}

impl Rect {
  /// Create from size.
  ///
  /// Checks that width and height are greater than zero.
  pub fn from_size((x, y): (u16, u16), (width, height): (u16, u16)) -> Option<Self> {
    if width > 0 && height > 0 {
      Some(Self::from_size_unchecked((x, y), (width, height)))
    } else {
      None
    }
  }

  /// Create from size.
  ///
  /// Assumes width and height are greater than zero.
  /// If unsure, call the "checked" function.
  pub fn from_size_unchecked((x, y): (u16, u16), (width, height): (u16, u16)) -> Self {
    Self {
      x: (x, x + width - 1),
      y: (y, y + height - 1),
    }
  }

  /// Get width or number of columns
  pub fn width(&self) -> u16 {
    self.x.1 - self.x.0 + 1
  }

  /// Get height or number of rows
  pub fn height(&self) -> u16 {
    self.y.1 - self.y.0 + 1
  }

  /// Get width or number of columns
  pub fn cols(&self) -> u16 {
    self.width()
  }

  /// Get height or number of rows
  pub fn rows(&self) -> u16 {
    self.height()
  }

  /// Return an inner Rect.
  ///
  /// Checks that the inner rectangle will be valid.
  pub fn inner(&self) -> Option<Self> {
    if !self.valid() || self.width() < 3 || self.height() < 3 {
      None
    } else {
      Some(self.inner_unchecked())
    }
  }

  /// Return an inner Rect.
  ///
  /// It is assumed current width and height are at least 3.
  /// If unsure, use the "checked" version.
  pub fn inner_unchecked(&self) -> Self {
    Self {
      x: (self.x.0 + 1, self.x.1 - 1),
      y: (self.y.0 + 1, self.y.1 - 1),
    }
  }

  /// Return an inner X axis Rect.
  ///
  /// Checks that the inner rectangle will be valid.
  pub fn inner_x(&self) -> Option<Self> {
    if !self.valid() || self.width() < 3 {
      None
    } else {
      Some(self.inner_x_unchecked())
    }
  }

  /// Return an inner X axis Rect.
  ///
  /// It is assumed current width is at least 3.
  /// If unsure, use the "checked" version.
  pub fn inner_x_unchecked(&self) -> Self {
    Self {
      x: (self.x.0 + 1, self.x.1 - 1),
      y: self.y,
    }
  }

  /// Return an outer Rect.
  ///
  /// Checks that the inner rectangle will be valid.
  pub fn outer(&self) -> Option<Self> {
    if !self.valid() || self.x.0 == 0 || self.y.0 == 0 {
      None
    } else {
      Some(self.outer_unchecked())
    }
  }

  /// Return an outer Rect.
  ///
  /// It is assumed current x0 and y0 are not at origin (0,0) but offset by at least 1.
  /// If unsure, use the "checked" version.
  pub fn outer_unchecked(&self) -> Self {
    Self {
      x: (self.x.0 - 1, self.x.1 + 1),
      y: (self.y.0 - 1, self.y.1 + 1),
    }
  }

  /// Return itself if Rect is still valid, otherwise consume itself and return None.
  pub fn checked(self) -> Option<Self> {
    if self.valid() {
      Some(self)
    } else {
      None
    }
  }

  /// Check if Self is a valid Rect.
  pub fn valid(&self) -> bool {
    self.x.0 <= self.x.1 && self.y.0 <= self.y.1
  }

  /// Check if Self is an invalid Rect.
  pub fn invalid(&self) -> bool {
    self.x.0 > self.x.1 || self.y.0 > self.y.1
  }

  /// Get Rect with new x.0 value.
  ///
  /// Checks that the new rectangle will be valid.
  pub fn with_x0(&self, x0: u16) -> Option<Self> {
    self.with_x0_unchecked(x0).checked()
  }

  /// Get Rect with new x.0 value.
  ///
  /// It is assumed current the new x0 value keeps the rectangle valid.
  /// If unsure, use the "checked" version.
  pub fn with_x0_unchecked(&self, x0: u16) -> Self {
    Self {
      x: (x0, self.x.1),
      y: self.y,
    }
  }

  /// Get Rect with new y.0 value.
  ///
  /// Checks that the new rectangle will be valid.
  pub fn with_y0(&self, y0: u16) -> Option<Self> {
    self.with_y0_unchecked(y0).checked()
  }

  /// Get Rect with new y.0 value.
  ///
  /// It is assumed current the new y0 value keeps the rectangle valid.
  /// If unsure, use the "checked" version.
  pub fn with_y0_unchecked(&self, y0: u16) -> Self {
    Self {
      x: self.x,
      y: (y0, self.y.1),
    }
  }

  /// Offset the y.0 value.
  ///
  /// Checks that the offset rectangle will be valid.
  pub fn offset_y0(&self, offset: i16) -> Option<Self> {
    let y0 = self.y.0 as i32 + offset as i32;
    y0.to_u16().and_then(|y0| self.with_y0_unchecked(y0).checked())
  }

  /// Offset the y.0 value.
  ///
  /// It is assumed current the offset value is within the y0 offset range.
  /// If unsure, use the "checked" version.
  pub fn offset_y0_unchecked(&self, offset: i16) -> Self {
    let y0 = self.y.0 as i32 + offset as i32;
    self.with_y0_unchecked(y0 as u16)
  }

  /// Offset the x.0 value.
  ///
  /// Checks that the offset rectangle will be valid.
  pub fn offset_x0(&self, offset: i16) -> Option<Self> {
    let x0 = self.x.0 as i32 + offset as i32;
    x0.to_u16().and_then(|x0| self.with_x0_unchecked(x0).checked())
  }

  /// Offset the x.0 value.
  ///
  /// It is assumed current the offset value is within the x0 offset range.
  /// If unsure, use the "checked" version.
  pub fn offset_x0_unchecked(&self, offset: i16) -> Self {
    let x0 = self.x.0 as i32 + offset as i32;
    self.with_x0_unchecked(x0 as u16)
  }

  /// Vertically split current Rect into two.
  ///
  /// In case the current width is odd, the left Rect takes the larger width.
  pub fn vsplit(self) -> Option<(Rect, Rect)> {
    if self.invalid() || self.width() < 2 {
      return None;
    }
    let left = Rect {
      x: (self.x.0, self.x.1 / 2),
      y: self.y.clone(),
    };
    let right = Rect {
      x: (left.x.1 + 1, self.x.1),
      y: self.y.clone(),
    };
    Some((left, right))
  }

  /// Horizontally split current Rect into two.
  ///
  /// In case the current height is odd, the first/top Rect takes the larger height.
  pub fn hsplit(self) -> Option<(Rect, Rect)> {
    if self.invalid() || self.height() < 2 {
      return None;
    }
    let top = Rect {
      x: self.x.clone(),
      y: (self.y.0, self.y.1 / 2),
    };
    let bottom = Rect {
      x: self.x.clone(),
      y: (top.y.1 + 1, self.y.1),
    };
    Some((top, bottom))
  }
}
