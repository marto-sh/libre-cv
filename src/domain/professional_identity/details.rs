use std::ops::Deref;

use snafu::Snafu;

use super::value_objects::{Detail, DetailId, SessionId, Source, TurnId};

#[derive(Debug, Snafu)]
#[snafu(module(detail_error), visibility(pub(crate)))]
pub enum DetailError {
    #[snafu(display("detail not found: {id}"))]
    NotFound { id: DetailId },
    #[snafu(display("source not found on detail {detail_id}: session {session_id}, turn {turn_id}"))]
    SourceNotFound {
        detail_id: DetailId,
        session_id: SessionId,
        turn_id: TurnId,
    },
}

#[derive(Debug, Clone)]
pub struct Details(Vec<Detail>);

impl Deref for Details {
    type Target = [Detail];

    fn deref(&self) -> &[Detail] {
        &self.0
    }
}

impl Details {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn add(&mut self, title: &str, text: &str) -> DetailId {
        let id = DetailId::new();
        self.0.push(Detail {
            id: id.clone(),
            title: title.to_string(),
            text: text.to_string(),
            sources: Vec::new(),
        });
        id
    }

    pub(super) fn update(
        &mut self,
        id: &DetailId,
        title: &str,
        text: &str,
    ) -> Result<(), DetailError> {
        let detail = self
            .0
            .iter_mut()
            .find(|d| &d.id == id)
            .ok_or_else(|| DetailError::NotFound { id: id.clone() })?;
        detail.title = title.to_string();
        detail.text = text.to_string();
        Ok(())
    }

    pub(super) fn add_source(
        &mut self,
        id: &DetailId,
        source: Source,
    ) -> Result<(), DetailError> {
        let detail = self
            .0
            .iter_mut()
            .find(|d| &d.id == id)
            .ok_or_else(|| DetailError::NotFound { id: id.clone() })?;
        detail.sources.push(source);
        Ok(())
    }

    pub(super) fn remove_source(
        &mut self,
        id: &DetailId,
        session_id: &SessionId,
        turn_id: &TurnId,
    ) -> Result<(), DetailError> {
        let detail = self
            .0
            .iter_mut()
            .find(|d| &d.id == id)
            .ok_or_else(|| DetailError::NotFound { id: id.clone() })?;
        let index = detail
            .sources
            .iter()
            .position(|s| &s.session == session_id && &s.turn == turn_id)
            .ok_or_else(|| DetailError::SourceNotFound {
                detail_id: id.clone(),
                session_id: session_id.clone(),
                turn_id: turn_id.clone(),
            })?;
        detail.sources.remove(index);
        Ok(())
    }

    pub(super) fn remove(&mut self, id: &DetailId) -> Result<(), DetailError> {
        let index = self
            .0
            .iter()
            .position(|d| &d.id == id)
            .ok_or_else(|| DetailError::NotFound { id: id.clone() })?;
        self.0.remove(index);
        Ok(())
    }
}
