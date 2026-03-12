use cucumber::{given, then, when};
use libre_cv::domain::professional_identity::value_objects::{SessionId, Source, TurnId};

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

// --- Add source to detail ---

#[when(
    expr = "the Owner adds a source from session {string} turn {string} to the skill detail"
)]
fn add_source_to_skill_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world.current_skill_id.as_ref().expect("should have a current skill");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_skill_detail(skill_id, detail_id, source)
        .expect("should add source");
}

#[when(
    expr = "the Owner adds a source from session {string} turn {string} to the experience detail"
)]
fn add_source_to_experience_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let experience_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_experience_detail(experience_id, detail_id, source)
        .expect("should add source");
}

#[when(
    expr = "the Owner adds a source from session {string} turn {string} to the project detail"
)]
fn add_source_to_project_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_project_detail(project_id, detail_id, source)
        .expect("should add source");
}

#[when(
    expr = "the Owner adds a source from session {string} turn {string} to the expectation detail"
)]
fn add_source_to_expectation_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_expectation_detail(expectation_id, detail_id, source)
        .expect("should add source");
}

// --- Given: source already added (setup for remove scenarios) ---

#[given(
    expr = "the Owner has added a source from session {string} turn {string} to the skill detail"
)]
fn given_source_added_to_skill_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world.current_skill_id.as_ref().expect("should have a current skill");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_skill_detail(skill_id, detail_id, source)
        .expect("should add source");
}

#[given(
    expr = "the Owner has added a source from session {string} turn {string} to the experience detail"
)]
fn given_source_added_to_experience_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let experience_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_experience_detail(experience_id, detail_id, source)
        .expect("should add source");
}

#[given(
    expr = "the Owner has added a source from session {string} turn {string} to the project detail"
)]
fn given_source_added_to_project_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_project_detail(project_id, detail_id, source)
        .expect("should add source");
}

#[given(
    expr = "the Owner has added a source from session {string} turn {string} to the expectation detail"
)]
fn given_source_added_to_expectation_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let source = Source {
        session: SessionId::new(&session).expect("valid session id"),
        turn: TurnId::new(&turn).expect("valid turn id"),
    };
    identity
        .add_source_to_expectation_detail(expectation_id, detail_id, source)
        .expect("should add source");
}

// --- Remove source from detail ---

#[when(
    expr = "the Owner removes the source from session {string} turn {string} from the skill detail"
)]
fn remove_source_from_skill_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world.current_skill_id.as_ref().expect("should have a current skill");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let session_id = SessionId::new(&session).expect("valid session id");
    let turn_id = TurnId::new(&turn).expect("valid turn id");
    match identity.remove_source_from_skill_detail(skill_id, detail_id, &session_id, &turn_id) {
        Ok(()) => {}
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when(
    expr = "the Owner removes the source from session {string} turn {string} from the experience detail"
)]
fn remove_source_from_experience_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let experience_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let session_id = SessionId::new(&session).expect("valid session id");
    let turn_id = TurnId::new(&turn).expect("valid turn id");
    match identity.remove_source_from_experience_detail(
        experience_id,
        detail_id,
        &session_id,
        &turn_id,
    ) {
        Ok(()) => {}
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when(
    expr = "the Owner removes the source from session {string} turn {string} from the project detail"
)]
fn remove_source_from_project_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let session_id = SessionId::new(&session).expect("valid session id");
    let turn_id = TurnId::new(&turn).expect("valid turn id");
    match identity.remove_source_from_project_detail(project_id, detail_id, &session_id, &turn_id)
    {
        Ok(()) => {}
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when(
    expr = "the Owner removes the source from session {string} turn {string} from the expectation detail"
)]
fn remove_source_from_expectation_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let session_id = SessionId::new(&session).expect("valid session id");
    let turn_id = TurnId::new(&turn).expect("valid turn id");
    match identity.remove_source_from_expectation_detail(
        expectation_id,
        detail_id,
        &session_id,
        &turn_id,
    ) {
        Ok(()) => {}
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[then("the source should not be removed")]
fn source_should_not_be_removed(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when removing source"
    );
}

// --- Source validation ---

#[when(
    expr = "the Owner tries to add a source from session {string} turn {string} to the skill detail"
)]
fn try_add_source_to_skill_detail(
    world: &mut ProfessionalIdentityWorld,
    session: String,
    turn: String,
) {
    let session_result = SessionId::new(&session);
    let turn_result = TurnId::new(&turn);
    match (session_result, turn_result) {
        (Ok(session), Ok(turn)) => {
            let identity = world.identity.as_mut().expect("identity should exist");
            let skill_id = world.current_skill_id.as_ref().expect("should have a current skill");
            let detail_id =
                world.current_detail_id.as_ref().expect("should have a current detail");
            let source = Source { session, turn };
            if let Err(e) = identity.add_source_to_skill_detail(skill_id, detail_id, source) {
                world.last_error = Some(e.to_string());
            }
        }
        (Err(e), _) => world.last_error = Some(e.to_string()),
        (_, Err(e)) => world.last_error = Some(e.to_string()),
    }
}

#[then("the source should not be added")]
fn source_should_not_be_added(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when adding source"
    );
}

#[then(expr = "the skill detail should have {int} source(s)")]
fn skill_detail_should_have_n_sources(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world.current_skill_id.as_ref().expect("should have a current skill");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let skill = identity.skill(skill_id).expect("skill should exist");
    let detail = skill.details.iter().find(|d| &d.id == detail_id).expect("detail should exist");
    assert_eq!(detail.sources.len(), count);
}

#[then(expr = "the experience detail should have {int} source(s)")]
fn experience_detail_should_have_n_sources(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let experience_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let experience = identity.experience(experience_id).expect("experience should exist");
    let detail = experience
        .details
        .iter()
        .find(|d| &d.id == detail_id)
        .expect("detail should exist");
    assert_eq!(detail.sources.len(), count);
}

#[then(expr = "the project detail should have {int} source(s)")]
fn project_detail_should_have_n_sources(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let project = identity.project(project_id).expect("project should exist");
    let detail = project
        .details
        .iter()
        .find(|d| &d.id == detail_id)
        .expect("detail should exist");
    assert_eq!(detail.sources.len(), count);
}

#[then(expr = "the expectation detail should have {int} source(s)")]
fn expectation_detail_should_have_n_sources(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = world.current_detail_id.as_ref().expect("should have a current detail");
    let expectation = identity.expectation(expectation_id).expect("expectation should exist");
    let detail = expectation
        .details
        .iter()
        .find(|d| &d.id == detail_id)
        .expect("detail should exist");
    assert_eq!(detail.sources.len(), count);
}
