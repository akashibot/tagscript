// This type is used for associating actions with the type
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Message(String),
    Stop(bool),
}
