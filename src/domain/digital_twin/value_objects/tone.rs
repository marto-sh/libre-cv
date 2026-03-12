use super::super::error::digital_twin_error::EmptyToneSnafu;
use super::super::error::DigitalTwinError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tone(String);

impl Tone {
    pub fn new(value: &str) -> Result<Self, DigitalTwinError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return EmptyToneSnafu.fail();
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
