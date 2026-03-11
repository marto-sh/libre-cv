use super::details::Details;
use super::entities::Project;
use super::error::ProjectError;
use super::value_objects::{DetailId, ExperienceId, ProjectId, SkillId};

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

    pub(super) fn position(&self, id: &ProjectId) -> Result<usize, ProjectError> {
        self.0
            .iter()
            .position(|p| &p.id == id)
            .ok_or_else(|| ProjectError::NotFound { id: id.clone() })
    }

    pub(super) fn get_by_index(&self, index: usize) -> &Project {
        &self.0[index]
    }

    pub(super) fn get_mut_by_index(&mut self, index: usize) -> &mut Project {
        &mut self.0[index]
    }

    pub(super) fn add_detail(
        &mut self,
        project_id: &ProjectId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ProjectError> {
        let project = self.get_mut(project_id)?;
        Ok(project.details.add(title, text))
    }

    pub(super) fn update_detail(
        &mut self,
        project_id: &ProjectId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ProjectError> {
        let project = self.get_mut(project_id)?;
        project
            .details
            .update(detail_id, title, text)
            .map_err(|source| ProjectError::Detail { source })
    }

    pub(super) fn remove_detail(
        &mut self,
        project_id: &ProjectId,
        detail_id: &DetailId,
    ) -> Result<(), ProjectError> {
        let project = self.get_mut(project_id)?;
        project
            .details
            .remove(detail_id)
            .map_err(|source| ProjectError::Detail { source })
    }

    pub(super) fn remove_skill_refs(&mut self, skill_id: &SkillId) {
        for project in &mut self.0 {
            project.skills.retain(|sid| sid != skill_id);
        }
    }

    pub(super) fn remove_experience_refs(&mut self, experience_id: &ExperienceId) {
        for project in &mut self.0 {
            if project.experience.as_ref() == Some(experience_id) {
                project.experience = None;
            }
        }
    }
}
