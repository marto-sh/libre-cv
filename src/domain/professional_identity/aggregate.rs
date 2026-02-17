use uuid::Uuid;

use super::value_objects::Name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessionalIdentityId(Uuid);

#[derive(Debug)]
pub struct ProfessionalIdentity {
    id: ProfessionalIdentityId,
    name: Name,
}

impl ProfessionalIdentity {
    pub fn draft(name: Name) -> Self {
        Self {
            id: ProfessionalIdentityId(Uuid::new_v4()),
            name,
        }
    }

    pub fn id(&self) -> &ProfessionalIdentityId {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
