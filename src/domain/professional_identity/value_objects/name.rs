use super::super::error::professional_identity_error::EmptyNameSnafu;
use super::super::error::ProfessionalIdentityError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new(value: &str) -> Result<Self, ProfessionalIdentityError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return EmptyNameSnafu.fail();
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
