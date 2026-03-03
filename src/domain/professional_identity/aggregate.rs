use jiff::Timestamp;
use uuid::Uuid;

use super::value_objects::{
    Detail, Expectation, Experience, ExperienceId, Name, Project, SessionId, Skill,
};

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

    headline: Option<String>,
    summary: Option<String>,

    experiences: Vec<Experience>,
    projects: Vec<Project>,
    skills: Vec<Skill>,
    expectations: Vec<Expectation>,

    sessions: Vec<SessionId>,
}

impl ProfessionalIdentity {
    pub fn draft(name: Name) -> Self {
        Self {
            id: ProfessionalIdentityId(Uuid::new_v4()),
            name,
            published_at: None,
            headline: None,
            summary: None,
            experiences: Vec::new(),
            projects: Vec::new(),
            skills: Vec::new(),
            expectations: Vec::new(),
            sessions: Vec::new(),
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

    // --- Experience CRUD ---

    pub fn add_experience(
        &mut self,
        role: &str,
        organization: Option<&str>,
    ) -> Result<ExperienceId, &'static str> {
        let role = role.trim();
        if role.is_empty() {
            return Err("Experience role must not be empty");
        }
        let id = ExperienceId::new();
        self.experiences.push(Experience {
            id: id.clone(),
            role: role.to_string(),
            organization: organization.map(|o| o.trim().to_string()),
            period: None,
            summary: String::new(),
            details: Vec::new(),
            skills: Vec::new(),
        });
        Ok(id)
    }

    pub fn update_experience_role(
        &mut self,
        id: &ExperienceId,
        role: &str,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(id)?;
        let role = role.trim();
        if role.is_empty() {
            return Err("Experience role must not be empty");
        }
        experience.role = role.to_string();
        Ok(())
    }

    pub fn update_experience_organization(
        &mut self,
        id: &ExperienceId,
        organization: &str,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(id)?;
        experience.organization = Some(organization.trim().to_string());
        Ok(())
    }

    pub fn remove_experience(&mut self, id: &ExperienceId) -> Result<(), &'static str> {
        let index = self
            .experiences
            .iter()
            .position(|e| &e.id == id)
            .ok_or("Experience not found")?;
        self.experiences.remove(index);
        Ok(())
    }

    pub fn experiences(&self) -> &[Experience] {
        &self.experiences
    }

    pub fn experience(&self, id: &ExperienceId) -> Option<&Experience> {
        self.experiences.iter().find(|e| &e.id == id)
    }

    // --- Detail CRUD on Experience ---

    pub fn add_detail_to_experience(
        &mut self,
        experience_id: &ExperienceId,
        text: &str,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(experience_id)?;
        experience.details.push(Detail {
            text: text.to_string(),
            sources: Vec::new(),
        });
        Ok(())
    }

    pub fn update_detail_on_experience(
        &mut self,
        experience_id: &ExperienceId,
        detail_index: usize,
        text: &str,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(experience_id)?;
        let detail = experience
            .details
            .get_mut(detail_index)
            .ok_or("Detail not found")?;
        detail.text = text.to_string();
        Ok(())
    }

    pub fn remove_detail_from_experience(
        &mut self,
        experience_id: &ExperienceId,
        detail_index: usize,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(experience_id)?;
        if detail_index >= experience.details.len() {
            return Err("Detail not found");
        }
        experience.details.remove(detail_index);
        Ok(())
    }

    fn experience_mut(&mut self, id: &ExperienceId) -> Result<&mut Experience, &'static str> {
        self.experiences
            .iter_mut()
            .find(|e| &e.id == id)
            .ok_or("Experience not found")
    }
}
