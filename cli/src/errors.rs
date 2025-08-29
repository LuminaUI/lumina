#[derive(Hash, Eq, PartialEq)]
pub enum Errors {
    MissingDirOrEmptyProject = 1,
    NoProjectInfo = 2,
    ImportAliasMissing = 3,
}
