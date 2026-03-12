use super::SessionId;
use super::TurnId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub session: SessionId,
    pub turn: TurnId,
}
