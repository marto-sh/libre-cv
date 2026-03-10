use cucumber::{given, then, when};

use crate::ProfessionalIdentityWorld;

// --- Add ---

#[when(expr = "the Owner adds an experience with role {string}")]
fn add_experience_with_role(world: &mut ProfessionalIdentityWorld, role: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match identity.add_experience(&role, None) {
        Ok(id) => world.current_experience_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when(expr = "the Owner adds an experience with role {string} at {string}")]
fn add_experience_with_role_and_org(
    world: &mut ProfessionalIdentityWorld,
    role: String,
    organization: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match identity.add_experience(&role, Some(&organization)) {
        Ok(id) => world.current_experience_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[given(expr = "the Owner has added an experience with role {string} at {string}")]
fn given_experience_added(
    world: &mut ProfessionalIdentityWorld,
    role: String,
    organization: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = identity
        .add_experience(&role, Some(&organization))
        .expect("should add experience");
    world.current_experience_id = Some(id);
}

// --- Update ---

#[when(expr = "the Owner updates the experience role to {string}")]
fn update_experience_role(world: &mut ProfessionalIdentityWorld, role: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    identity
        .update_experience_role(id, &role)
        .expect("should update role");
}

#[when(expr = "the Owner updates the experience organization to {string}")]
fn update_experience_organization(world: &mut ProfessionalIdentityWorld, organization: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    identity
        .update_experience_organization(id, &organization)
        .expect("should update organization");
}

// --- Remove ---

#[when("the Owner removes the experience")]
fn remove_experience(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_experience_id
        .take()
        .expect("should have a current experience");
    identity
        .remove_experience(&id)
        .expect("should remove experience");
}

// --- Details ---

#[when(expr = "the Owner adds a detail {string} to the experience")]
fn add_detail_to_experience(world: &mut ProfessionalIdentityWorld, text: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = identity
        .add_detail_to_experience(id, &text)
        .expect("should add detail");
    world.current_detail_id = Some(detail_id);
}

#[given(expr = "the Owner has added a detail {string} to the experience")]
fn given_detail_added(world: &mut ProfessionalIdentityWorld, text: String) {
    add_detail_to_experience(world, text);
}

#[when(expr = "the Owner updates the detail text to {string}")]
fn update_detail_text(world: &mut ProfessionalIdentityWorld, text: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = world
        .current_detail_id
        .as_ref()
        .expect("should have a current detail");
    identity
        .update_detail_on_experience(id, detail_id, &text)
        .expect("should update detail");
}

#[when("the Owner removes the detail from the experience")]
fn remove_detail(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let detail_id = world
        .current_detail_id
        .as_ref()
        .expect("should have a current detail");
    identity
        .remove_detail_from_experience(id, detail_id)
        .expect("should remove detail");
}

// --- Assertions ---

#[then(expr = "the Professional Identity should have {int} experience(s)")]
fn should_have_n_experiences(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.experiences().len(), count);
}

#[then(expr = "the experience should have the role {string}")]
fn experience_should_have_role(world: &mut ProfessionalIdentityWorld, expected_role: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let experience = identity.experience(id).expect("experience should exist");
    assert_eq!(experience.role, expected_role);
}

#[then(expr = "the experience should have the organization {string}")]
fn experience_should_have_org(world: &mut ProfessionalIdentityWorld, expected_org: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let experience = identity.experience(id).expect("experience should exist");
    assert_eq!(
        experience.organization.as_deref(),
        Some(expected_org.as_str())
    );
}

#[then("the experience should not be added")]
fn experience_should_not_be_added(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when adding experience"
    );
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.experiences().len(), 0);
}

#[then(expr = "the experience should have {int} detail(s)")]
fn experience_should_have_n_details(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let experience = identity.experience(id).expect("experience should exist");
    assert_eq!(experience.details.len(), count);
}

#[then(expr = "the detail should have the text {string}")]
fn detail_should_have_text(world: &mut ProfessionalIdentityWorld, expected_text: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let experience = identity.experience(id).expect("experience should exist");
    let detail = experience.details.first().expect("should have a detail");
    assert_eq!(detail.text, expected_text);
}
