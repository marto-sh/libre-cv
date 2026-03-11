mod detail_id;
mod expectation_id;
mod experience_id;
mod project_id;
mod section_locator;
mod session_id;
mod skill_id;
mod source;

pub use detail_id::DetailId;
pub use expectation_id::ExpectationId;
pub use experience_id::ExperienceId;
pub use project_id::ProjectId;
pub use section_locator::SectionLocator;
pub use session_id::SessionId;
pub use skill_id::SkillId;
pub use source::Source;

use jiff::civil::Date;

use super::error::professional_identity_error::EmptyNameSnafu;
use super::error::ProfessionalIdentityError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detail {
    pub id: DetailId,
    pub title: String,
    pub text: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new(value: &str) -> Result<Self, ProfessionalIdentityError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return EmptyNameSnafu.fail();
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    pub start: Date,
    pub end: Option<Date>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectationKind {
    Constraint,
    Preference,
}
