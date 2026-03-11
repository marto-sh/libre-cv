use super::SectionLocator;
use super::SessionId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub session: SessionId,
    pub section: SectionLocator,
}
