use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectationId(Uuid);

impl ExpectationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
