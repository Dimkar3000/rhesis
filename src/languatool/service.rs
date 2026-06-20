use crate::interop::bridge::ffi::Recommendation;

#[derive(Debug, Clone)]
pub struct Message(pub String);
#[derive(Debug, Clone, PartialEq)]
pub struct Suggestion(pub Vec<Recommendation>);
