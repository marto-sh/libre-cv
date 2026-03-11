use super::super::details::Details;
use super::super::value_objects::{ExperienceId, ProjectId, SkillId};

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub experience: Option<ExperienceId>,
    pub details: Details,
    pub skills: Vec<SkillId>,
}
