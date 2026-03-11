mod experience_id;
mod project_id;
mod skill_id;

pub use experience_id::ExperienceId;
pub use project_id::ProjectId;
pub use skill_id::SkillId;

use std::fmt;

use jiff::civil::Date;
use uuid::Uuid;

use super::error::professional_identity_error::EmptyNameSnafu;
use super::error::ProfessionalIdentityError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetailId(Uuid);

impl DetailId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for DetailId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectationId(Uuid);

impl ExpectationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

// Placeholder — will be refined when libre-session is integrated
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(String);

// Placeholder — depends on libre-session's representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionLocator(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub session: SessionId,
    pub section: SectionLocator,
}

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
