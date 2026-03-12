use cucumber::{given, then, when};
use libre_cv::domain::professional_identity::value_objects::SessionId;

use crate::ProfessionalIdentityWorld;

// --- Add session ---

#[when(expr = "the Owner adds a session {string}")]
fn add_session(world: &mut ProfessionalIdentityWorld, session_id: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match SessionId::new(&session_id) {
        Ok(sid) => identity.add_session(sid),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[given(expr = "the Owner has added a session {string}")]
fn given_session_added(world: &mut ProfessionalIdentityWorld, session_id: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let sid = SessionId::new(&session_id).expect("valid session id");
    identity.add_session(sid);
}

#[then("the session should not be added")]
fn session_should_not_be_added(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when adding session"
    );
}

// --- Remove session ---

#[when(expr = "the Owner removes the session {string}")]
fn remove_session(world: &mut ProfessionalIdentityWorld, session_id: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let sid = SessionId::new(&session_id).expect("valid session id");
    match identity.remove_session(&sid) {
        Ok(()) => {}
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[then("the session should not be removed")]
fn session_should_not_be_removed(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when removing session"
    );
}

#[then(expr = "the Professional Identity should have {int} session(s)")]
fn should_have_n_sessions(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.sessions().len(), count);
}
