pub mod accounts;
pub mod changes;
pub mod details;

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Gerrit Interface
#[derive(Default, Debug)]
pub struct Gerrit {
    host: String,
    username: String,
    password: String,
}
