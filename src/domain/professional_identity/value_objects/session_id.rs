use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: &str) -> Result<Self, EmptySessionId> {
        let id = id.trim();
        if id.is_empty() {
            return Err(EmptySessionId);
        }
        Ok(Self(id.to_string()))
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct EmptySessionId;

impl fmt::Display for EmptySessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "session id must not be empty")
    }
}

impl std::error::Error for EmptySessionId {}
