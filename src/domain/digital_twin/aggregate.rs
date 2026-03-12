use snafu::ResultExt;

use super::error::digital_twin_error::ToneSnafu;
use super::error::DigitalTwinError;
use super::value_objects::{DigitalTwinId, Tone};
use crate::domain::professional_identity::aggregate::ProfessionalIdentityId;

#[derive(Debug)]
pub struct DigitalTwin {
    id: DigitalTwinId,
    professional_identity_id: ProfessionalIdentityId,
    tone: Option<Tone>,
}

impl DigitalTwin {
    pub fn create(professional_identity_id: ProfessionalIdentityId) -> Self {
        Self {
            id: DigitalTwinId::generate(),
            professional_identity_id,
            tone: None,
        }
    }

    pub fn id(&self) -> &DigitalTwinId {
        &self.id
    }

    pub fn professional_identity_id(&self) -> &ProfessionalIdentityId {
        &self.professional_identity_id
    }

    pub fn set_tone(&mut self, instruction: &str) -> Result<(), DigitalTwinError> {
        let tone = Tone::new(instruction).context(ToneSnafu)?;
        self.tone = Some(tone);
        Ok(())
    }

    pub fn clear_tone(&mut self) {
        self.tone = None;
    }

    pub fn tone(&self) -> Option<&Tone> {
        self.tone.as_ref()
    }
}
