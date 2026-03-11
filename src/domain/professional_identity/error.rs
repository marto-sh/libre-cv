use snafu::Snafu;

use super::details::DetailError;
use super::value_objects::{ExperienceId, ProjectId, SkillId};

#[derive(Debug, Snafu)]
#[snafu(module(experience_error), visibility(pub(crate)))]
pub enum ExperienceError {
    #[snafu(display("experience not found: {id}"))]
    NotFound { id: ExperienceId },
    #[snafu(display("experience role must not be empty"))]
    EmptyRole,
    #[snafu(display("{source}"))]
    Detail { source: DetailError },
}

#[derive(Debug, Snafu)]
#[snafu(module(skill_error), visibility(pub(crate)))]
pub enum SkillError {
    #[snafu(display("skill not found: {id}"))]
    NotFound { id: SkillId },
    #[snafu(display("skill name must not be empty"))]
    EmptyName,
    #[snafu(display("{source}"))]
    Detail { source: DetailError },
}

#[derive(Debug, Snafu)]
#[snafu(module(project_error), visibility(pub(crate)))]
pub enum ProjectError {
    #[snafu(display("project not found: {id}"))]
    NotFound { id: ProjectId },
    #[snafu(display("project name must not be empty"))]
    EmptyName,
    #[snafu(display("{source}"))]
    Detail { source: DetailError },
}

#[derive(Debug, Snafu)]
#[snafu(module(professional_identity_error), visibility(pub(crate)))]
pub enum ProfessionalIdentityError {
    #[snafu(display("{source}"))]
    Experience { source: ExperienceError },
    #[snafu(display("{source}"))]
    Project { source: ProjectError },
    #[snafu(display("{source}"))]
    Skill { source: SkillError },
    #[snafu(display("name must not be empty"))]
    EmptyName,
    #[snafu(display("headline must not be empty"))]
    EmptyHeadline,
    #[snafu(display("summary must not be empty"))]
    EmptySummary,
}
