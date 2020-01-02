/// Validate a string is convertible to u32
pub fn is_u32(v: String) -> Result<(), String> {
    if v.parse::<u32>().is_ok() { return Ok(()); }
    Err(String::from("not a number"))
}
