use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnId(String);

impl TurnId {
    pub fn new(id: &str) -> Result<Self, EmptyTurnId> {
        let id = id.trim();
        if id.is_empty() {
            return Err(EmptyTurnId);
        }
        Ok(Self(id.to_string()))
    }
}

impl fmt::Display for TurnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct EmptyTurnId;

impl fmt::Display for EmptyTurnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "turn id must not be empty")
    }
}

impl std::error::Error for EmptyTurnId {}
