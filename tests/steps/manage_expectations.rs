use cucumber::{given, then, when};
use libre_cv::domain::professional_identity::value_objects::ExpectationKind;

use crate::ProfessionalIdentityWorld;

// --- Add ---

#[when(expr = "the Owner adds a constraint named {string}")]
fn add_constraint(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match identity.add_expectation(&name, ExpectationKind::Constraint) {
        Ok(id) => world.current_expectation_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when(expr = "the Owner adds a preference named {string}")]
fn add_preference(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match identity.add_expectation(&name, ExpectationKind::Preference) {
        Ok(id) => world.current_expectation_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[given(expr = "the Owner has added a constraint named {string}")]
fn given_constraint_added(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = identity
        .add_expectation(&name, ExpectationKind::Constraint)
        .expect("should add constraint");
    world.current_expectation_id = Some(id);
}

#[given(expr = "the Owner has added a preference named {string}")]
fn given_preference_added(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = identity
        .add_expectation(&name, ExpectationKind::Preference)
        .expect("should add preference");
    world.current_expectation_id = Some(id);
}

// --- Update ---

#[when(expr = "the Owner updates the expectation name to {string}")]
fn update_expectation_name(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    identity
        .update_expectation_name(id, &name)
        .expect("should update expectation name");
}

// --- Assertions ---

#[then(expr = "the Professional Identity should have {int} expectation(s)")]
fn should_have_n_expectations(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.expectations().len(), count);
}

#[then(expr = "the expectation should have the name {string}")]
fn expectation_should_have_name(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let expectation = identity.expectation(id).expect("expectation should exist");
    assert_eq!(expectation.name, expected);
}

#[then("the expectation should be a Constraint")]
fn expectation_should_be_constraint(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let expectation = identity.expectation(id).expect("expectation should exist");
    assert_eq!(expectation.kind, ExpectationKind::Constraint);
}

#[then("the expectation should be a Preference")]
fn expectation_should_be_preference(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let expectation = identity.expectation(id).expect("expectation should exist");
    assert_eq!(expectation.kind, ExpectationKind::Preference);
}

#[then("the expectation should not be added")]
fn expectation_should_not_be_added(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when adding expectation"
    );
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.expectations().len(), 0);
}
