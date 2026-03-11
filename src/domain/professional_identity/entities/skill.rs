use super::super::details::Details;
use super::super::value_objects::{ExpectationId, ExperienceId, ProjectId, SkillId};

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub details: Details,
    pub experiences: Vec<ExperienceId>,
    pub projects: Vec<ProjectId>,
    pub expectations: Vec<ExpectationId>,
}
