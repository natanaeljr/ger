pub struct Remote {
    pub name: String,
    pub url: String,
    pub username: String,
    pub http_password: String,
}

pub struct Config {
    pub remotes: Vec<Remote>,
}
