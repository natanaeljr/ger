pub mod validate;

/// Get default port for the protocol in the URL
pub fn default_port_for_url(url: &str) -> u16 {
    if url.starts_with("https://") {
        443
    } else if url.starts_with("http://") {
        80
    } else {
        panic!()
    }
}
