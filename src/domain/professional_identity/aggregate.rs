use jiff::Timestamp;
use snafu::ResultExt;
use uuid::Uuid;

use super::error::professional_identity_error::{
    ExperienceSnafu, ExpectationSnafu, ProjectSnafu, SkillSnafu,
};
use super::error::ProfessionalIdentityError;
use super::expectations::Expectations;
use super::experiences::Experiences;
use super::projects::Projects;
use super::skills::Skills;
use super::entities::{Expectation, Experience, Project, Skill};
use super::value_objects::{
    DetailId, ExpectationId, ExpectationKind, ExperienceId, Name, ProjectId, SessionId, SkillId,
    Source, TurnId,
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

    experiences: Experiences,
    projects: Projects,
    skills: Skills,
    expectations: Expectations,

    session_ids: Vec<SessionId>,
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
            expectations: Expectations::new(),
            session_ids: Vec::new(),
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

    pub fn add_session(&mut self, session: SessionId) {
        if !self.session_ids.contains(&session) {
            self.session_ids.push(session);
        }
    }

    pub fn remove_session(&mut self, session: &SessionId) -> Result<(), ProfessionalIdentityError> {
        let index = self
            .session_ids
            .iter()
            .position(|s| s == session)
            .ok_or_else(|| ProfessionalIdentityError::SessionNotFound {
                id: session.clone(),
            })?;
        self.session_ids.remove(index);
        Ok(())
    }

    pub fn session_ids(&self) -> &[SessionId] {
        &self.session_ids
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
        self.experiences.remove(id).context(ExperienceSnafu)?;
        self.projects.remove_experience_refs(id);
        self.expectations.remove_experience_refs(id);
        Ok(())
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

    pub fn add_project(
        &mut self,
        name: &str,
        experience_id: Option<&ExperienceId>,
    ) -> Result<ProjectId, ProfessionalIdentityError> {
        let eid = if let Some(id) = experience_id {
            let _ = self
                .experiences
                .get(id)
                .ok_or_else(|| super::error::ExperienceError::NotFound { id: id.clone() })
                .context(ExperienceSnafu)?;
            Some(id.clone())
        } else {
            None
        };
        self.projects.add(name, eid).context(ProjectSnafu)
    }

    pub fn projects(&self) -> &[Project] {
        self.projects.list()
    }

    pub fn project(&self, id: &ProjectId) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn remove_project(
        &mut self,
        id: &ProjectId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.projects.remove(id).context(ProjectSnafu)
    }

    pub fn update_project_name(
        &mut self,
        id: &ProjectId,
        name: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.projects.update_name(id, name).context(ProjectSnafu)
    }

    pub fn add_detail_to_project(
        &mut self,
        project_id: &ProjectId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ProfessionalIdentityError> {
        self.projects
            .add_detail(project_id, title, text)
            .context(ProjectSnafu)
    }

    pub fn remove_detail_from_project(
        &mut self,
        project_id: &ProjectId,
        detail_id: &DetailId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.projects
            .remove_detail(project_id, detail_id)
            .context(ProjectSnafu)
    }

    pub fn update_detail_on_project(
        &mut self,
        project_id: &ProjectId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.projects
            .update_detail(project_id, detail_id, title, text)
            .context(ProjectSnafu)
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
        self.projects.remove_skill_refs(&skill_id);
        self.expectations.remove_skill_refs(&skill_id);
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

    pub fn link_skill_to_project(
        &mut self,
        skill_id: &SkillId,
        project_id: &ProjectId,
    ) -> Result<(), ProfessionalIdentityError> {
        let skill_idx = self.skills.position(skill_id).context(SkillSnafu)?;
        let project_idx = self.projects.position(project_id).context(ProjectSnafu)?;

        let sid = self.skills.get_by_index(skill_idx).id.clone();
        let pid = self.projects.get_by_index(project_idx).id.clone();

        if !self.skills.get_by_index(skill_idx).projects.contains(&pid) {
            self.skills.get_mut_by_index(skill_idx).projects.push(pid);
        }
        if !self.projects.get_by_index(project_idx).skills.contains(&sid) {
            self.projects.get_mut_by_index(project_idx).skills.push(sid);
        }
        Ok(())
    }

    pub fn unlink_skill_from_project(
        &mut self,
        skill_id: &SkillId,
        project_id: &ProjectId,
    ) -> Result<(), ProfessionalIdentityError> {
        let skill_idx = self.skills.position(skill_id).context(SkillSnafu)?;
        let project_idx = self.projects.position(project_id).context(ProjectSnafu)?;

        self.skills
            .get_mut_by_index(skill_idx)
            .projects
            .retain(|pid| pid != project_id);
        self.projects
            .get_mut_by_index(project_idx)
            .skills
            .retain(|sid| sid != skill_id);
        Ok(())
    }

    pub fn add_expectation(
        &mut self,
        name: &str,
        kind: ExpectationKind,
    ) -> Result<ExpectationId, ProfessionalIdentityError> {
        self.expectations.add(name, kind).context(ExpectationSnafu)
    }

    pub fn remove_expectation(
        &mut self,
        id: &ExpectationId,
    ) -> Result<(), ProfessionalIdentityError> {
        let expectation_id = self.expectations.remove(id).context(ExpectationSnafu)?;
        self.skills.remove_expectation_refs(&expectation_id);
        self.experiences.remove_expectation_refs(&expectation_id);
        Ok(())
    }

    pub fn update_expectation_name(
        &mut self,
        id: &ExpectationId,
        name: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.expectations
            .update_name(id, name)
            .context(ExpectationSnafu)
    }

    pub fn update_expectation_kind(
        &mut self,
        id: &ExpectationId,
        kind: ExpectationKind,
    ) -> Result<(), ProfessionalIdentityError> {
        self.expectations
            .update_kind(id, kind)
            .context(ExpectationSnafu)
    }

    pub fn expectations(&self) -> &[Expectation] {
        self.expectations.list()
    }

    pub fn expectation(&self, id: &ExpectationId) -> Option<&Expectation> {
        self.expectations.get(id)
    }

    pub fn link_skill_to_expectation(
        &mut self,
        skill_id: &SkillId,
        expectation_id: &ExpectationId,
    ) -> Result<(), ProfessionalIdentityError> {
        let skill_idx = self.skills.position(skill_id).context(SkillSnafu)?;
        let expectation_idx = self
            .expectations
            .position(expectation_id)
            .context(ExpectationSnafu)?;

        let sid = self.skills.get_by_index(skill_idx).id.clone();
        let eid = self.expectations.get_by_index(expectation_idx).id.clone();

        if !self
            .skills
            .get_by_index(skill_idx)
            .expectations
            .contains(&eid)
        {
            self.skills
                .get_mut_by_index(skill_idx)
                .expectations
                .push(eid);
        }
        if !self
            .expectations
            .get_by_index(expectation_idx)
            .skills
            .contains(&sid)
        {
            self.expectations
                .get_mut_by_index(expectation_idx)
                .skills
                .push(sid);
        }
        Ok(())
    }

    pub fn link_experience_to_expectation(
        &mut self,
        experience_id: &ExperienceId,
        expectation_id: &ExpectationId,
    ) -> Result<(), ProfessionalIdentityError> {
        let experience = self
            .experiences
            .get(experience_id)
            .ok_or_else(|| super::error::ExperienceError::NotFound {
                id: experience_id.clone(),
            })
            .context(ExperienceSnafu)?;
        let expectation_idx = self
            .expectations
            .position(expectation_id)
            .context(ExpectationSnafu)?;

        let exp_id = experience.id.clone();
        let eid = self.expectations.get_by_index(expectation_idx).id.clone();

        if !self
            .expectations
            .get_by_index(expectation_idx)
            .experiences
            .contains(&exp_id)
        {
            self.expectations
                .get_mut_by_index(expectation_idx)
                .experiences
                .push(exp_id);
        }
        let experience = self.experiences.get_mut(experience_id).unwrap();
        if !experience.expectations.contains(&eid) {
            experience.expectations.push(eid);
        }
        Ok(())
    }

    pub fn unlink_experience_from_expectation(
        &mut self,
        experience_id: &ExperienceId,
        expectation_id: &ExpectationId,
    ) -> Result<(), ProfessionalIdentityError> {
        let _ = self
            .experiences
            .get(experience_id)
            .ok_or_else(|| super::error::ExperienceError::NotFound {
                id: experience_id.clone(),
            })
            .context(ExperienceSnafu)?;
        let expectation_idx = self
            .expectations
            .position(expectation_id)
            .context(ExpectationSnafu)?;

        self.expectations
            .get_mut_by_index(expectation_idx)
            .experiences
            .retain(|eid| eid != experience_id);
        let experience = self.experiences.get_mut(experience_id).unwrap();
        experience
            .expectations
            .retain(|eid| eid != expectation_id);
        Ok(())
    }

    pub fn unlink_skill_from_expectation(
        &mut self,
        skill_id: &SkillId,
        expectation_id: &ExpectationId,
    ) -> Result<(), ProfessionalIdentityError> {
        let skill_idx = self.skills.position(skill_id).context(SkillSnafu)?;
        let expectation_idx = self
            .expectations
            .position(expectation_id)
            .context(ExpectationSnafu)?;

        self.skills
            .get_mut_by_index(skill_idx)
            .expectations
            .retain(|eid| eid != expectation_id);
        self.expectations
            .get_mut_by_index(expectation_idx)
            .skills
            .retain(|sid| sid != skill_id);
        Ok(())
    }

    pub fn remove_detail_from_expectation(
        &mut self,
        expectation_id: &ExpectationId,
        detail_id: &DetailId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.expectations
            .remove_detail(expectation_id, detail_id)
            .context(ExpectationSnafu)
    }

    pub fn update_detail_on_expectation(
        &mut self,
        expectation_id: &ExpectationId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ProfessionalIdentityError> {
        self.expectations
            .update_detail(expectation_id, detail_id, title, text)
            .context(ExpectationSnafu)
    }

    pub fn add_detail_to_expectation(
        &mut self,
        expectation_id: &ExpectationId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ProfessionalIdentityError> {
        self.expectations
            .add_detail(expectation_id, title, text)
            .context(ExpectationSnafu)
    }

    pub fn add_source_to_skill_detail(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        source: Source,
    ) -> Result<(), ProfessionalIdentityError> {
        self.skills
            .add_source_to_detail(skill_id, detail_id, source)
            .context(SkillSnafu)
    }

    pub fn add_source_to_experience_detail(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        source: Source,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences
            .add_source_to_detail(experience_id, detail_id, source)
            .context(ExperienceSnafu)
    }

    pub fn add_source_to_project_detail(
        &mut self,
        project_id: &ProjectId,
        detail_id: &DetailId,
        source: Source,
    ) -> Result<(), ProfessionalIdentityError> {
        self.projects
            .add_source_to_detail(project_id, detail_id, source)
            .context(ProjectSnafu)
    }

    pub fn add_source_to_expectation_detail(
        &mut self,
        expectation_id: &ExpectationId,
        detail_id: &DetailId,
        source: Source,
    ) -> Result<(), ProfessionalIdentityError> {
        self.expectations
            .add_source_to_detail(expectation_id, detail_id, source)
            .context(ExpectationSnafu)
    }

    pub fn remove_source_from_skill_detail(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.skills
            .remove_source_from_detail(skill_id, detail_id, session_id, turn_id)
            .context(SkillSnafu)
    }

    pub fn remove_source_from_experience_detail(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.experiences
            .remove_source_from_detail(experience_id, detail_id, session_id, turn_id)
            .context(ExperienceSnafu)
    }

    pub fn remove_source_from_project_detail(
        &mut self,
        project_id: &ProjectId,
        detail_id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.projects
            .remove_source_from_detail(project_id, detail_id, session_id, turn_id)
            .context(ProjectSnafu)
    }

    pub fn remove_source_from_expectation_detail(
        &mut self,
        expectation_id: &ExpectationId,
        detail_id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), ProfessionalIdentityError> {
        self.expectations
            .remove_source_from_detail(expectation_id, detail_id, session_id, turn_id)
            .context(ExpectationSnafu)
    }
}
