use jiff::Timestamp;
use snafu::ResultExt;
use uuid::Uuid;

use super::error::professional_identity_error::{ExperienceSnafu, SkillSnafu};
use super::error::ProfessionalIdentityError;
use super::experiences::Experiences;
use super::projects::Projects;
use super::skills::Skills;
use super::entities::{Expectation, Experience, Skill};
use super::value_objects::{DetailId, ExperienceId, Name, SessionId, SkillId};

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

    experiences: Experiences,
    projects: Projects,
    skills: Skills,
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
            experiences: Experiences::new(),
            projects: Projects::new(),
            skills: Skills::new(),
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

    pub fn set_headline(&mut self, headline: &str) -> Result<(), ProfessionalIdentityError> {
        let headline = headline.trim();
        if headline.is_empty() {
            return Err(ProfessionalIdentityError::EmptyHeadline);
        }
        self.headline = Some(headline.to_string());
        Ok(())
    }

    pub fn clear_headline(&mut self) {
        self.headline = None;
    }

    pub fn headline(&self) -> Option<&str> {
        self.headline.as_deref()
    }

    pub fn set_summary(&mut self, summary: &str) -> Result<(), ProfessionalIdentityError> {
        let summary = summary.trim();
        if summary.is_empty() {
            return Err(ProfessionalIdentityError::EmptySummary);
        }
        self.summary = Some(summary.to_string());
        Ok(())
    }

    pub fn clear_summary(&mut self) {
        self.summary = None;
    }

    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    pub fn add_experience(
        &mut self,
        role: &str,
        organization: Option<&str>,
    ) -> Result<ExperienceId, ProfessionalIdentityError> {
        self.experiences
            .add(role, organization)
            .context(ExperienceSnafu)
    }

    pub fn update_experience_role(
        &mut self,
        id: &ExperienceId,
        role: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences.update_role(id, role).context(ExperienceSnafu)
    }

    pub fn update_experience_organization(
        &mut self,
        id: &ExperienceId,
        organization: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences
            .update_organization(id, organization)
            .context(ExperienceSnafu)
    }

    pub fn remove_experience(
        &mut self,
        id: &ExperienceId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences.remove(id).context(ExperienceSnafu)
    }

    pub fn experiences(&self) -> &[Experience] {
        self.experiences.list()
    }

    pub fn experience(&self, id: &ExperienceId) -> Option<&Experience> {
        self.experiences.get(id)
    }

    pub fn add_detail_to_experience(
        &mut self,
        experience_id: &ExperienceId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ProfessionalIdentityError> {
        self.experiences
            .add_detail(experience_id, title, text)
            .context(ExperienceSnafu)
    }

    pub fn update_detail_on_experience(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences
            .update_detail(experience_id, detail_id, title, text)
            .context(ExperienceSnafu)
    }

    pub fn remove_detail_from_experience(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences
            .remove_detail(experience_id, detail_id)
            .context(ExperienceSnafu)
    }

    pub fn add_skill(
        &mut self,
        name: &str,
    ) -> Result<SkillId, ProfessionalIdentityError> {
        self.skills.add(name).context(SkillSnafu)
    }

    pub fn update_skill_name(
        &mut self,
        id: &SkillId,
        name: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.skills.update_name(id, name).context(SkillSnafu)
    }

    pub fn remove_skill(&mut self, id: &SkillId) -> Result<(), ProfessionalIdentityError> {
        let skill_id = self.skills.remove(id).context(SkillSnafu)?;
        self.experiences.remove_skill_refs(&skill_id);
        Ok(())
    }

    pub fn skills(&self) -> &[Skill] {
        self.skills.list()
    }

    pub fn skill(&self, id: &SkillId) -> Option<&Skill> {
        self.skills.get(id)
    }

    pub fn add_detail_to_skill(
        &mut self,
        skill_id: &SkillId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ProfessionalIdentityError> {
        self.skills
            .add_detail(skill_id, title, text)
            .context(SkillSnafu)
    }

    pub fn update_detail_on_skill(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.skills
            .update_detail(skill_id, detail_id, title, text)
            .context(SkillSnafu)
    }

    pub fn remove_detail_from_skill(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.skills
            .remove_detail(skill_id, detail_id)
            .context(SkillSnafu)
    }

    pub fn link_skill_to_experience(
        &mut self,
        skill_id: &SkillId,
        experience_id: &ExperienceId,
    ) -> Result<(), ProfessionalIdentityError> {
        let skill_idx = self.skills.position(skill_id).context(SkillSnafu)?;
        let experience = self
            .experiences
            .get(experience_id)
            .ok_or_else(|| super::error::ExperienceError::NotFound {
                id: experience_id.clone(),
            })
            .context(ExperienceSnafu)?;

        let sid = self.skills.get_by_index(skill_idx).id.clone();
        let eid = experience.id.clone();

        if !self.skills.get_by_index(skill_idx).experiences.contains(&eid) {
            self.skills.get_mut_by_index(skill_idx).experiences.push(eid);
        }
        let experience = self.experiences.get_mut(experience_id).unwrap();
        if !experience.skills.contains(&sid) {
            experience.skills.push(sid);
        }
        Ok(())
    }

    pub fn unlink_skill_from_experience(
        &mut self,
        skill_id: &SkillId,
        experience_id: &ExperienceId,
    ) -> Result<(), ProfessionalIdentityError> {
        let skill_idx = self.skills.position(skill_id).context(SkillSnafu)?;
        let _ = self
            .experiences
            .get(experience_id)
            .ok_or_else(|| super::error::ExperienceError::NotFound {
                id: experience_id.clone(),
            })
            .context(ExperienceSnafu)?;

        self.skills
            .get_mut_by_index(skill_idx)
            .experiences
            .retain(|eid| eid != experience_id);
        let experience = self.experiences.get_mut(experience_id).unwrap();
        experience.skills.retain(|sid| sid != skill_id);
        Ok(())
    }
}
