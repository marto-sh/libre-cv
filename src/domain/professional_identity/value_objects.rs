use std::fmt;

use jiff::civil::Date;
use uuid::Uuid;

use super::error::professional_identity_error::EmptyNameSnafu;
use super::error::ProfessionalIdentityError;

// === IDs ===

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperienceId(Uuid);

impl ExperienceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ExperienceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillId(Uuid);

impl SkillId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for SkillId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

// === Traceability ===

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
    pub text: String,
    pub sources: Vec<Source>,
}

// === Value Objects ===

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

// === Facets ===

#[derive(Debug, Clone)]
pub struct Experience {
    pub id: ExperienceId,
    pub role: String,
    pub organization: Option<String>,
    pub period: Option<Period>,
    pub summary: String,
    pub details: Vec<Detail>,
    pub skills: Vec<SkillId>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub experience: Option<ExperienceId>,
    pub details: Vec<Detail>,
    pub skills: Vec<SkillId>,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub details: Vec<Detail>,
    pub experiences: Vec<ExperienceId>,
    pub projects: Vec<ProjectId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectationKind {
    Constraint,
    Preference,
}

#[derive(Debug, Clone)]
pub struct Expectation {
    pub id: ExpectationId,
    pub kind: ExpectationKind,
    pub name: String,
    pub details: Vec<Detail>,
}
