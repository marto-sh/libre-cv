use super::super::details::Details;
use super::super::value_objects::{ExperienceId, ExpectationId, ExpectationKind, SkillId};

#[derive(Debug, Clone)]
pub struct Expectation {
    pub id: ExpectationId,
    pub kind: ExpectationKind,
    pub name: String,
    pub details: Details,
    pub skills: Vec<SkillId>,
    pub experiences: Vec<ExperienceId>,
}
