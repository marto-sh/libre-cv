use super::details::Details;
use super::entities::Project;
use super::error::ProjectError;
use super::value_objects::{ExperienceId, ProjectId};

#[derive(Debug)]
pub(super) struct Projects(Vec<Project>);

impl Projects {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn add(
        &mut self,
        name: &str,
        experience_id: Option<ExperienceId>,
    ) -> Result<ProjectId, ProjectError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(ProjectError::EmptyName);
        }
        let id = ProjectId::new();
        self.0.push(Project {
            id: id.clone(),
            name: name.to_string(),
            experience: experience_id,
            details: Details::new(),
            skills: Vec::new(),
        });
        Ok(id)
    }

    pub(super) fn update_name(
        &mut self,
        id: &ProjectId,
        name: &str,
    ) -> Result<(), ProjectError> {
        let project = self.get_mut(id)?;
        let name = name.trim();
        if name.is_empty() {
            return Err(ProjectError::EmptyName);
        }
        project.name = name.to_string();
        Ok(())
    }

    pub(super) fn remove(&mut self, id: &ProjectId) -> Result<(), ProjectError> {
        let index = self
            .0
            .iter()
            .position(|p| &p.id == id)
            .ok_or_else(|| ProjectError::NotFound { id: id.clone() })?;
        self.0.remove(index);
        Ok(())
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
