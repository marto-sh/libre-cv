mod detail;
mod detail_id;
mod expectation_id;
mod experience_id;
mod name;
mod project_id;
mod section_locator;
mod session_id;
mod skill_id;
mod source;

pub use detail::Detail;
pub use detail_id::DetailId;
pub use expectation_id::ExpectationId;
pub use experience_id::ExperienceId;
pub use name::Name;
pub use project_id::ProjectId;
pub use section_locator::SectionLocator;
pub use session_id::SessionId;
pub use skill_id::SkillId;
pub use source::Source;

use jiff::civil::Date;

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
