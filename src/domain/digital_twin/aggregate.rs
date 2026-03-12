use super::value_objects::DigitalTwinId;
use crate::domain::professional_identity::aggregate::ProfessionalIdentityId;

#[derive(Debug)]
pub struct DigitalTwin {
    id: DigitalTwinId,
    professional_identity_id: ProfessionalIdentityId,
}
