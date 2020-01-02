use std::fmt;
use std::str::FromStr;

pub mod change;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    Change,
}

impl Command {
    pub fn variants() -> [&'static str; 1] {
        ["change"]
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "change" => Ok(Command::Change),
            _ => Err(String::from("[valid values: change]")),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Change => write!(f, "change"),
        }
    }
}
