use super::super::value_objects::{Detail, ExperienceId, ProjectId, SkillId};

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub experience: Option<ExperienceId>,
    pub details: Vec<Detail>,
    pub skills: Vec<SkillId>,
}
