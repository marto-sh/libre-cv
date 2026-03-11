use super::super::details::Details;
use super::super::value_objects::{ExpectationId, ExpectationKind};

#[derive(Debug, Clone)]
pub struct Expectation {
    pub id: ExpectationId,
    pub kind: ExpectationKind,
    pub name: String,
    pub details: Details,
}
