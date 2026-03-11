use super::super::value_objects::{Detail, ExpectationId, ExpectationKind};

#[derive(Debug, Clone)]
pub struct Expectation {
    pub id: ExpectationId,
    pub kind: ExpectationKind,
    pub name: String,
    pub details: Vec<Detail>,
}
