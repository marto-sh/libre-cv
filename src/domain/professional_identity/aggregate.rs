use jiff::Timestamp;
use uuid::Uuid;

use super::value_objects::{
    Detail, DetailId, Expectation, Experience, ExperienceId, Name, Project, SessionId, Skill,
    SkillId,
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
    ) -> Result<DetailId, &'static str> {
        let experience = self.experience_mut(experience_id)?;
        let id = DetailId::new();
        experience.details.push(Detail {
            id: id.clone(),
            text: text.to_string(),
            sources: Vec::new(),
        });
        Ok(id)
    }

    pub fn update_detail_on_experience(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
        text: &str,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(experience_id)?;
        let detail = experience
            .details
            .iter_mut()
            .find(|d| &d.id == detail_id)
            .ok_or("Detail not found")?;
        detail.text = text.to_string();
        Ok(())
    }

    pub fn remove_detail_from_experience(
        &mut self,
        experience_id: &ExperienceId,
        detail_id: &DetailId,
    ) -> Result<(), &'static str> {
        let experience = self.experience_mut(experience_id)?;
        let index = experience
            .details
            .iter()
            .position(|d| &d.id == detail_id)
            .ok_or("Detail not found")?;
        experience.details.remove(index);
        Ok(())
    }

    fn experience_mut(&mut self, id: &ExperienceId) -> Result<&mut Experience, &'static str> {
        self.experiences
            .iter_mut()
            .find(|e| &e.id == id)
            .ok_or("Experience not found")
    }

    // --- Skill CRUD ---

    pub fn add_skill(&mut self, name: &str) -> Result<SkillId, &'static str> {
        let name = name.trim();
        if name.is_empty() {
            return Err("Skill name must not be empty");
        }
        let id = SkillId::new();
        self.skills.push(Skill {
            id: id.clone(),
            name: name.to_string(),
            details: Vec::new(),
            experiences: Vec::new(),
            projects: Vec::new(),
        });
        Ok(id)
    }

    pub fn update_skill_name(
        &mut self,
        id: &SkillId,
        name: &str,
    ) -> Result<(), &'static str> {
        let skill = self.skill_mut(id)?;
        let name = name.trim();
        if name.is_empty() {
            return Err("Skill name must not be empty");
        }
        skill.name = name.to_string();
        Ok(())
    }

    pub fn remove_skill(&mut self, id: &SkillId) -> Result<(), &'static str> {
        let index = self
            .skills
            .iter()
            .position(|s| &s.id == id)
            .ok_or("Skill not found")?;
        let skill_id = self.skills[index].id.clone();
        self.skills.remove(index);
        for experience in &mut self.experiences {
            experience.skills.retain(|sid| sid != &skill_id);
        }
        Ok(())
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn skill(&self, id: &SkillId) -> Option<&Skill> {
        self.skills.iter().find(|s| &s.id == id)
    }

    // --- Detail CRUD on Skill ---

    pub fn add_detail_to_skill(
        &mut self,
        skill_id: &SkillId,
        text: &str,
    ) -> Result<DetailId, &'static str> {
        let skill = self.skill_mut(skill_id)?;
        let id = DetailId::new();
        skill.details.push(Detail {
            id: id.clone(),
            text: text.to_string(),
            sources: Vec::new(),
        });
        Ok(id)
    }

    pub fn update_detail_on_skill(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        text: &str,
    ) -> Result<(), &'static str> {
        let skill = self.skill_mut(skill_id)?;
        let detail = skill
            .details
            .iter_mut()
            .find(|d| &d.id == detail_id)
            .ok_or("Detail not found")?;
        detail.text = text.to_string();
        Ok(())
    }

    pub fn remove_detail_from_skill(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
    ) -> Result<(), &'static str> {
        let skill = self.skill_mut(skill_id)?;
        let index = skill
            .details
            .iter()
            .position(|d| &d.id == detail_id)
            .ok_or("Detail not found")?;
        skill.details.remove(index);
        Ok(())
    }

    fn skill_mut(&mut self, id: &SkillId) -> Result<&mut Skill, &'static str> {
        self.skills
            .iter_mut()
            .find(|s| &s.id == id)
            .ok_or("Skill not found")
    }

    // --- Skill ↔ Experience cross-references ---

    pub fn link_skill_to_experience(
        &mut self,
        skill_id: &SkillId,
        experience_id: &ExperienceId,
    ) -> Result<(), &'static str> {
        let skill_idx = self
            .skills
            .iter()
            .position(|s| &s.id == skill_id)
            .ok_or("Skill not found")?;
        let exp_idx = self
            .experiences
            .iter()
            .position(|e| &e.id == experience_id)
            .ok_or("Experience not found")?;

        let sid = self.skills[skill_idx].id.clone();
        let eid = self.experiences[exp_idx].id.clone();

        if !self.skills[skill_idx].experiences.contains(&eid) {
            self.skills[skill_idx].experiences.push(eid);
        }
        if !self.experiences[exp_idx].skills.contains(&sid) {
            self.experiences[exp_idx].skills.push(sid);
        }
        Ok(())
    }

    pub fn unlink_skill_from_experience(
        &mut self,
        skill_id: &SkillId,
        experience_id: &ExperienceId,
    ) -> Result<(), &'static str> {
        let skill_idx = self
            .skills
            .iter()
            .position(|s| &s.id == skill_id)
            .ok_or("Skill not found")?;
        let exp_idx = self
            .experiences
            .iter()
            .position(|e| &e.id == experience_id)
            .ok_or("Experience not found")?;

        self.skills[skill_idx]
            .experiences
            .retain(|eid| eid != experience_id);
        self.experiences[exp_idx]
            .skills
            .retain(|sid| sid != skill_id);
        Ok(())
    }
}
