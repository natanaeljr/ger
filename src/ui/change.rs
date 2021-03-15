#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeColumn {
    Commit = 0,
    Number = 1,
    Owner = 2,
    Time = 3,
    Project = 4,
    Branch = 5,
    Topic = 6,
    Status = 7,
    Subject = 8,
}
