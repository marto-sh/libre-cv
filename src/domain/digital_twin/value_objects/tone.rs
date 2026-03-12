use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tone(String);

impl Tone {
    pub fn new(value: &str) -> Result<Self, EmptyTone> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(EmptyTone);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct EmptyTone;

impl fmt::Display for EmptyTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tone must not be empty")
    }
}

impl std::error::Error for EmptyTone {}
