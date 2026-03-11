use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
