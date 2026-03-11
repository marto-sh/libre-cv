mod detail;
mod detail_id;
mod expectation_id;
mod experience_id;
mod name;
mod project_id;
mod section_locator;
mod session_id;
mod skill_id;
mod period;
mod source;

pub use detail::Detail;
pub use detail_id::DetailId;
pub use expectation_id::ExpectationId;
pub use experience_id::ExperienceId;
pub use name::Name;
pub use period::Period;
pub use project_id::ProjectId;
pub use section_locator::SectionLocator;
pub use session_id::SessionId;
pub use skill_id::SkillId;
pub use source::Source;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectationKind {
    Constraint,
    Preference,
}
