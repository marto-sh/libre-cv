use std::fmt;

use uuid::Uuid;

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
