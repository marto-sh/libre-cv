use super::DetailId;
use super::Source;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detail {
    pub id: DetailId,
    pub title: String,
    pub text: String,
    pub sources: Vec<Source>,
}
