use super::details::Details;
use super::entities::Expectation;
use super::error::ExpectationError;
use super::value_objects::{DetailId, ExperienceId, ExpectationId, ExpectationKind, SkillId};

#[derive(Debug)]
pub(super) struct Expectations(Vec<Expectation>);

impl Expectations {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn add(
        &mut self,
        name: &str,
        kind: ExpectationKind,
    ) -> Result<ExpectationId, ExpectationError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(ExpectationError::EmptyName);
        }
        let id = ExpectationId::new();
        self.0.push(Expectation {
            id: id.clone(),
            kind,
            name: name.to_string(),
            details: Details::new(),
            skills: Vec::new(),
            experiences: Vec::new(),
        });
        Ok(id)
    }

    pub(super) fn update_name(
        &mut self,
        id: &ExpectationId,
        name: &str,
    ) -> Result<(), ExpectationError> {
        let expectation = self.get_mut(id)?;
        let name = name.trim();
        if name.is_empty() {
            return Err(ExpectationError::EmptyName);
        }
        expectation.name = name.to_string();
        Ok(())
    }

    pub(super) fn update_kind(
        &mut self,
        id: &ExpectationId,
        kind: ExpectationKind,
    ) -> Result<(), ExpectationError> {
        let expectation = self.get_mut(id)?;
        expectation.kind = kind;
        Ok(())
    }

    pub(super) fn remove(&mut self, id: &ExpectationId) -> Result<ExpectationId, ExpectationError> {
        let index = self
            .0
            .iter()
            .position(|e| &e.id == id)
            .ok_or_else(|| ExpectationError::NotFound { id: id.clone() })?;
        let expectation_id = self.0[index].id.clone();
        self.0.remove(index);
        Ok(expectation_id)
    }

    pub(super) fn get(&self, id: &ExpectationId) -> Option<&Expectation> {
        self.0.iter().find(|e| &e.id == id)
    }

    pub(super) fn get_mut(
        &mut self,
        id: &ExpectationId,
    ) -> Result<&mut Expectation, ExpectationError> {
        self.0
            .iter_mut()
            .find(|e| &e.id == id)
            .ok_or_else(|| ExpectationError::NotFound { id: id.clone() })
    }

    pub(super) fn list(&self) -> &[Expectation] {
        &self.0
    }

    pub(super) fn position(&self, id: &ExpectationId) -> Result<usize, ExpectationError> {
        self.0
            .iter()
            .position(|e| &e.id == id)
            .ok_or_else(|| ExpectationError::NotFound { id: id.clone() })
    }

    pub(super) fn get_by_index(&self, index: usize) -> &Expectation {
        &self.0[index]
    }

    pub(super) fn get_mut_by_index(&mut self, index: usize) -> &mut Expectation {
        &mut self.0[index]
    }

    pub(super) fn add_detail(
        &mut self,
        expectation_id: &ExpectationId,
        title: &str,
        text: &str,
    ) -> Result<DetailId, ExpectationError> {
        let expectation = self.get_mut(expectation_id)?;
        Ok(expectation.details.add(title, text))
    }

    pub(super) fn update_detail(
        &mut self,
        expectation_id: &ExpectationId,
        detail_id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), ExpectationError> {
        let expectation = self.get_mut(expectation_id)?;
        expectation
            .details
            .update(detail_id, title, text)
            .map_err(|source| ExpectationError::Detail { source })
    }

    pub(super) fn remove_detail(
        &mut self,
        expectation_id: &ExpectationId,
        detail_id: &DetailId,
    ) -> Result<(), ExpectationError> {
        let expectation = self.get_mut(expectation_id)?;
        expectation
            .details
            .remove(detail_id)
            .map_err(|source| ExpectationError::Detail { source })
    }

    pub(super) fn remove_skill_refs(&mut self, skill_id: &SkillId) {
        for expectation in &mut self.0 {
            expectation.skills.retain(|sid| sid != skill_id);
        }
    }

    pub(super) fn remove_experience_refs(&mut self, experience_id: &ExperienceId) {
        for expectation in &mut self.0 {
            expectation.experiences.retain(|eid| eid != experience_id);
        }
    }
}
