/**************************************************************************************************/
/// Validate a string is convertible to u32
pub fn is_u32(v: String) -> Result<(), String> {
  if v.parse::<u32>().is_ok() {
    return Ok(());
  }
  Err(String::from("not a number"))
}

/// Validate a string is convertible to u16 in range
pub fn is_u16_range(v: String) -> Result<(), String> {
  if v.parse::<u16>().is_ok() {
    return Ok(());
  }
  Err(String::from("not a number in the range 0-65535"))
}

pub fn is_url_http_https(v: String) -> Result<(), String> {
  if v.starts_with("http://") || v.starts_with("https://") {
    return Ok(());
  }
  Err(String::from("missing http:// or https://"))
}
