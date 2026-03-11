use std::fmt;

use uuid::Uuid;

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
