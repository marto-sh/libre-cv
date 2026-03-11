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

// --- Remove ---

#[when("the Owner removes the expectation")]
fn remove_expectation(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .take()
        .expect("should have a current expectation");
    identity
        .remove_expectation(&id)
        .expect("should remove expectation");
}

#[when(expr = "the Owner changes the expectation kind to Constraint")]
fn change_to_constraint(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    identity
        .update_expectation_kind(id, ExpectationKind::Constraint)
        .expect("should update expectation kind");
}

#[when(expr = "the Owner changes the expectation kind to Preference")]
fn change_to_preference(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    identity
        .update_expectation_kind(id, ExpectationKind::Preference)
        .expect("should update expectation kind");
}

// --- Details ---

#[when(
    expr = "the Owner adds a detail titled {string} with text {string} to the expectation"
)]
fn add_detail_to_expectation(
    world: &mut ProfessionalIdentityWorld,
    title: String,
    text: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = identity
        .add_detail_to_expectation(id, &title, &text)
        .expect("should add detail");
    world.current_detail_id = Some(detail_id);
}

#[given(
    expr = "the Owner has added a detail titled {string} with text {string} to the expectation"
)]
fn given_detail_added_to_expectation(
    world: &mut ProfessionalIdentityWorld,
    title: String,
    text: String,
) {
    add_detail_to_expectation(world, title, text);
}

#[when(
    expr = "the Owner updates the expectation detail to title {string} and text {string}"
)]
fn update_expectation_detail(
    world: &mut ProfessionalIdentityWorld,
    title: String,
    text: String,
) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = world
        .current_detail_id
        .as_ref()
        .expect("should have a current detail");
    identity
        .update_detail_on_expectation(id, detail_id, &title, &text)
        .expect("should update detail");
}

// --- Cross-references ---

#[when("the Owner links the skill to the expectation")]
fn link_skill_to_expectation(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    identity
        .link_skill_to_expectation(skill_id, expectation_id)
        .expect("should link");
}

#[given("the Owner has linked the skill to the expectation")]
fn given_skill_linked_to_expectation(world: &mut ProfessionalIdentityWorld) {
    link_skill_to_expectation(world);
}

#[when("the Owner unlinks the skill from the expectation")]
fn unlink_skill_from_expectation(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    identity
        .unlink_skill_from_expectation(skill_id, expectation_id)
        .expect("should unlink");
}

#[when("the Owner removes the detail from the expectation")]
fn remove_detail_from_expectation(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let detail_id = world
        .current_detail_id
        .as_ref()
        .expect("should have a current detail");
    identity
        .remove_detail_from_expectation(id, detail_id)
        .expect("should remove detail");
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

#[then(expr = "the expectation should have {int} detail(s)")]
fn expectation_should_have_n_details(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let expectation = identity.expectation(id).expect("expectation should exist");
    assert_eq!(expectation.details.len(), count);
}

#[then(expr = "the expectation detail should have the title {string}")]
fn expectation_detail_should_have_title(
    world: &mut ProfessionalIdentityWorld,
    expected: String,
) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let expectation = identity.expectation(id).expect("expectation should exist");
    let detail = expectation.details.first().expect("should have a detail");
    assert_eq!(detail.title, expected);
}

#[then(expr = "the expectation detail should have the text {string}")]
fn expectation_detail_should_have_text(
    world: &mut ProfessionalIdentityWorld,
    expected: String,
) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let expectation = identity.expectation(id).expect("expectation should exist");
    let detail = expectation.details.first().expect("should have a detail");
    assert_eq!(detail.text, expected);
}

#[then("the expectation should reference the skill")]
fn expectation_should_reference_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let expectation = identity
        .expectation(expectation_id)
        .expect("expectation should exist");
    assert!(
        expectation.skills.contains(skill_id),
        "Expectation should reference the skill"
    );
}

#[then("the skill should reference the expectation")]
fn skill_should_reference_expectation(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let skill = identity.skill(skill_id).expect("skill should exist");
    assert!(
        skill.expectations.contains(expectation_id),
        "Skill should reference the expectation"
    );
}

#[then("the expectation should not reference the skill")]
fn expectation_should_not_reference_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let expectation = identity
        .expectation(expectation_id)
        .expect("expectation should exist");
    assert!(
        !expectation.skills.contains(skill_id),
        "Expectation should not reference the skill"
    );
}

#[then("the skill should not reference the expectation")]
fn skill_should_not_reference_expectation(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let expectation_id = world
        .current_expectation_id
        .as_ref()
        .expect("should have a current expectation");
    let skill = identity.skill(skill_id).expect("skill should exist");
    assert!(
        !skill.expectations.contains(expectation_id),
        "Skill should not reference the expectation"
    );
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
