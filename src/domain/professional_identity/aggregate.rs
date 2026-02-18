use jiff::Timestamp;
use uuid::Uuid;

use super::value_objects::Name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessionalIdentityId(Uuid);

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Draft,
    Published,
}

#[derive(Debug)]
pub struct ProfessionalIdentity {
    id: ProfessionalIdentityId,
    name: Name,
    published_at: Option<Timestamp>,
}

impl ProfessionalIdentity {
    pub fn draft(name: Name) -> Self {
        Self {
            id: ProfessionalIdentityId(Uuid::new_v4()),
            name,
            published_at: None,
        }
    }

    pub fn publish(&mut self) {
        self.published_at = Some(Timestamp::now());
    }

    pub fn unpublish(&mut self) {
        self.published_at = None;
    }

    pub fn status(&self) -> Status {
        match self.published_at {
            Some(_) => Status::Published,
            None => Status::Draft,
        }
    }

    pub fn id(&self) -> &ProfessionalIdentityId {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn published_at(&self) -> Option<&Timestamp> {
        self.published_at.as_ref()
    }
}
