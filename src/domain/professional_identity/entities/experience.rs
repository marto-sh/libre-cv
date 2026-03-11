use super::super::details::Details;
use super::super::value_objects::{ExperienceId, Period, SkillId};

#[derive(Debug, Clone)]
pub struct Experience {
    pub id: ExperienceId,
    pub role: String,
    pub organization: Option<String>,
    pub period: Option<Period>,
    pub summary: String,
    pub details: Details,
    pub skills: Vec<SkillId>,
}
