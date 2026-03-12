use std::fmt;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalTwinId(Uuid);

impl DigitalTwinId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for DigitalTwinId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
