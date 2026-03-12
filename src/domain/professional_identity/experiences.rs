use super::details::Details;
use super::entities::Experience;
use super::error::ExperienceError;
use super::value_objects::{DetailId, ExpectationId, ExperienceId, SessionId, SkillId, Source, TurnId};

#[derive(Debug)]
pub(super) struct Experiences(Vec<Experience>);

impl Experiences {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn add(
        &mut self,
        role: &str,
        organization: Option<&str>,
    ) -> Result<ExperienceId, ExperienceError> {
        let role = role.trim();
        if role.is_empty() {
            return Err(ExperienceError::EmptyRole);
        }
        let id = ExperienceId::new();
        self.0.push(Experience {
            id: id.clone(),
            role: role.to_string(),
            organization: organization.map(|o| o.trim().to_string()),
            period: None,
            summary: String::new(),
            details: Details::new(),
            skills: Vec::new(),
            expectations: Vec::new(),
        });
        Ok(id)
    }

    pub(super) fn update_role(
        &mut self,
        id: &ExperienceId,
        role: &str,
    ) -> Result<(), ExperienceError> {
        let experience = self.get_mut(id)?;
        let role = role.trim();
        if role.is_empty() {
            return Err(ExperienceError::EmptyRole);
        }
        experience.role = role.to_string();
        Ok(())
    }

    pub(super) fn update_organization(
        &mut self,
        id: &ExperienceId,
        organization: &str,
    ) -> Result<(), ExperienceError> {
        let experience = self.get_mut(id)?;
        experience.organization = Some(organization.trim().to_string());
        Ok(())
    }

    pub(super) fn remove(&mut self, id: &ExperienceId) -> Result<(), ExperienceError> {
        let index = self
            .0
            .iter()
            .position(|e| &e.id == id)
            .ok_or_else(|| ExperienceError::NotFound { id: id.clone() })?;
        self.0.remove(index);
        Ok(())
    }

    pub(super) fn get(&self, id: &ExperienceId) -> Option<&Experience> {
        self.0.iter().find(|e| &e.id == id)
    }

    pub(super) fn get_mut(
        &mut self,
        id: &ExperienceId,
    ) -> Result<&mut Experience, ExperienceError> {
        self.0
            .iter_mut()
            .find(|e| &e.id == id)
            .ok_or_else(|| ExperienceError::NotFound { id: id.clone() })
    }

    pub(super) fn list(&self) -> &[Experience] {
        &self.0
    }

    pub(super) fn remove_skill_refs(&mut self, skill_id: &SkillId) {
        for experience in &mut self.0 {
            experience.skills.retain(|sid| sid != skill_id);
        }
    }

    pub(super) fn add_detail(
        &mut self,
        experience_id: &ExperienceId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ExperienceError> {
        let experience = self.get_mut(experience_id)?;
        Ok(experience.details.add(title, text))
    }

    pub(super) fn update_detail(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ExperienceError> {
        let experience = self.get_mut(experience_id)?;
        experience
            .details
            .update(detail_id, title, text)
            .map_err(|source| ExperienceError::Detail { source })
    }

    pub(super) fn add_source_to_detail(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        source: Source,
    ) -> Result<(), ExperienceError> {
        let experience = self.get_mut(experience_id)?;
        experience
            .details
            .add_source(detail_id, source)
            .map_err(|source| ExperienceError::Detail { source })
    }

    pub(super) fn remove_source_from_detail(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), ExperienceError> {
        let experience = self.get_mut(experience_id)?;
        experience
            .details
            .remove_source(detail_id, session_id, turn_id)
            .map_err(|source| ExperienceError::Detail { source })
    }

    pub(super) fn remove_detail(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
    ) -> Result<(), ExperienceError> {
        let experience = self.get_mut(experience_id)?;
        experience
            .details
            .remove(detail_id)
            .map_err(|source| ExperienceError::Detail { source })
    }

    pub(super) fn remove_expectation_refs(&mut self, expectation_id: &ExpectationId) {
        for experience in &mut self.0 {
            experience
                .expectations
                .retain(|eid| eid != expectation_id);
        }
    }
}
