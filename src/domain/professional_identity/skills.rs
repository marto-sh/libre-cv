use super::details::Details;
use super::entities::Skill;
use super::error::SkillError;
use super::value_objects::{DetailId, ExpectationId, SessionId, SkillId, Source, TurnId};

#[derive(Debug)]
pub(super) struct Skills(Vec<Skill>);

impl Skills {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn add(&mut self, name: &str) -> Result<SkillId, SkillError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(SkillError::EmptyName);
        }
        let id = SkillId::new();
        self.0.push(Skill {
            id: id.clone(),
            name: name.to_string(),
            details: Details::new(),
            experiences: Vec::new(),
            projects: Vec::new(),
            expectations: Vec::new(),
        });
        Ok(id)
    }

    pub(super) fn update_name(
        &mut self,
        id: &SkillId,
        name: &str,
    ) -> Result<(), SkillError> {
        let skill = self.get_mut(id)?;
        let name = name.trim();
        if name.is_empty() {
            return Err(SkillError::EmptyName);
        }
        skill.name = name.to_string();
        Ok(())
    }

    pub(super) fn remove(&mut self, id: &SkillId) -> Result<SkillId, SkillError> {
        let index = self
            .0
            .iter()
            .position(|s| &s.id == id)
            .ok_or_else(|| SkillError::NotFound { id: id.clone() })?;
        let skill_id = self.0[index].id.clone();
        self.0.remove(index);
        Ok(skill_id)
    }

    pub(super) fn get(&self, id: &SkillId) -> Option<&Skill> {
        self.0.iter().find(|s| &s.id == id)
    }

    pub(super) fn get_mut(&mut self, id: &SkillId) -> Result<&mut Skill, SkillError> {
        self.0
            .iter_mut()
            .find(|s| &s.id == id)
            .ok_or_else(|| SkillError::NotFound { id: id.clone() })
    }

    pub(super) fn list(&self) -> &[Skill] {
        &self.0
    }

    pub(super) fn position(&self, id: &SkillId) -> Result<usize, SkillError> {
        self.0
            .iter()
            .position(|s| &s.id == id)
            .ok_or_else(|| SkillError::NotFound { id: id.clone() })
    }

    pub(super) fn get_by_index(&self, index: usize) -> &Skill {
        &self.0[index]
    }

    pub(super) fn get_mut_by_index(&mut self, index: usize) -> &mut Skill {
        &mut self.0[index]
    }

    pub(super) fn add_detail(
        &mut self,
        skill_id: &SkillId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, SkillError> {
        let skill = self.get_mut(skill_id)?;
        Ok(skill.details.add(title, text))
    }

    pub(super) fn update_detail(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), SkillError> {
        let skill = self.get_mut(skill_id)?;
        skill
            .details
            .update(detail_id, title, text)
            .map_err(|source| SkillError::Detail { source })
    }

    pub(super) fn add_source_to_detail(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        source: Source,
    ) -> Result<(), SkillError> {
        let skill = self.get_mut(skill_id)?;
        skill
            .details
            .add_source(detail_id, source)
            .map_err(|source| SkillError::Detail { source })
    }

    pub(super) fn remove_source_from_detail(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), SkillError> {
        let skill = self.get_mut(skill_id)?;
        skill
            .details
            .remove_source(detail_id, session_id, turn_id)
            .map_err(|source| SkillError::Detail { source })
    }

    pub(super) fn remove_detail(
        &mut self,
        skill_id: &SkillId,
        detail_id: &DetailId,
    ) -> Result<(), SkillError> {
        let skill = self.get_mut(skill_id)?;
        skill
            .details
            .remove(detail_id)
            .map_err(|source| SkillError::Detail { source })
    }

    pub(super) fn remove_expectation_refs(&mut self, expectation_id: &ExpectationId) {
        for skill in &mut self.0 {
            skill.expectations.retain(|eid| eid != expectation_id);
        }
    }
}
