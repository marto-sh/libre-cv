use super::entities::Project;
use super::error::ProjectError;
use super::value_objects::ProjectId;

#[derive(Debug)]
pub(super) struct Projects(Vec<Project>);

impl Projects {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn get(&self, id: &ProjectId) -> Option<&Project> {
        self.0.iter().find(|p| &p.id == id)
    }

    pub(super) fn get_mut(&mut self, id: &ProjectId) -> Result<&mut Project, ProjectError> {
        self.0
            .iter_mut()
            .find(|p| &p.id == id)
            .ok_or_else(|| ProjectError::NotFound { id: id.clone() })
    }

    pub(super) fn list(&self) -> &[Project] {
        &self.0
    }
}
