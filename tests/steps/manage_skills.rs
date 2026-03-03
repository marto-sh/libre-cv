use cucumber::{given, then, when};

use crate::ProfessionalIdentityWorld;

// --- Add ---

#[when(expr = "the Owner adds a skill named {string}")]
fn add_skill(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match identity.add_skill(&name) {
        Ok(id) => world.current_skill_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[given(expr = "the Owner has added a skill named {string}")]
fn given_skill_added(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = identity.add_skill(&name).expect("should add skill");
    world.current_skill_id = Some(id);
}

// --- Update ---

#[when(expr = "the Owner updates the skill name to {string}")]
fn update_skill_name(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    identity
        .update_skill_name(id, &name)
        .expect("should update skill name");
}

// --- Remove ---

#[when("the Owner removes the skill")]
fn remove_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_skill_id
        .take()
        .expect("should have a current skill");
    identity.remove_skill(&id).expect("should remove skill");
}

// --- Details ---

#[when(expr = "the Owner adds a detail {string} to the skill")]
fn add_detail_to_skill(world: &mut ProfessionalIdentityWorld, text: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    identity
        .add_detail_to_skill(id, &text)
        .expect("should add detail");
}

#[given(expr = "the Owner has added a detail {string} to the skill")]
fn given_detail_added_to_skill(world: &mut ProfessionalIdentityWorld, text: String) {
    add_detail_to_skill(world, text);
}

#[when(expr = "the Owner updates the skill detail text to {string}")]
fn update_skill_detail_text(world: &mut ProfessionalIdentityWorld, text: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    identity
        .update_detail_on_skill(id, 0, &text)
        .expect("should update detail");
}

#[when("the Owner removes the detail from the skill")]
fn remove_detail_from_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    identity
        .remove_detail_from_skill(id, 0)
        .expect("should remove detail");
}

// --- Cross-references ---

#[when("the Owner links the skill to the experience")]
fn link_skill_to_experience(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    identity
        .link_skill_to_experience(skill_id, exp_id)
        .expect("should link");
}

#[given("the Owner has linked the skill to the experience")]
fn given_skill_linked(world: &mut ProfessionalIdentityWorld) {
    link_skill_to_experience(world);
}

#[when("the Owner unlinks the skill from the experience")]
fn unlink_skill_from_experience(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    identity
        .unlink_skill_from_experience(skill_id, exp_id)
        .expect("should unlink");
}

// --- Assertions ---

#[then(expr = "the Professional Identity should have {int} skill(s)")]
fn should_have_n_skills(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.skills().len(), count);
}

#[then(expr = "the skill should have the name {string}")]
fn skill_should_have_name(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let skill = identity.skill(id).expect("skill should exist");
    assert_eq!(skill.name, expected);
}

#[then("the skill should not be added")]
fn skill_should_not_be_added(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when adding skill"
    );
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.skills().len(), 0);
}

#[then(expr = "the skill should have {int} detail(s)")]
fn skill_should_have_n_details(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let skill = identity.skill(id).expect("skill should exist");
    assert_eq!(skill.details.len(), count);
}

#[then(expr = "the skill detail should have the text {string}")]
fn skill_detail_should_have_text(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let skill = identity.skill(id).expect("skill should exist");
    let detail = skill.details.first().expect("should have a detail");
    assert_eq!(detail.text, expected);
}

#[then("the skill should reference the experience")]
fn skill_should_reference_experience(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let skill = identity.skill(skill_id).expect("skill should exist");
    assert!(
        skill.experiences.contains(exp_id),
        "Skill should reference the experience"
    );
}

#[then("the experience should reference the skill")]
fn experience_should_reference_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let experience = identity.experience(exp_id).expect("experience should exist");
    assert!(
        experience.skills.contains(skill_id),
        "Experience should reference the skill"
    );
}

#[then("the skill should not reference the experience")]
fn skill_should_not_reference_experience(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let skill = identity.skill(skill_id).expect("skill should exist");
    assert!(
        !skill.experiences.contains(exp_id),
        "Skill should not reference the experience"
    );
}

#[then("the experience should not reference the skill")]
fn experience_should_not_reference_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let experience = identity.experience(exp_id).expect("experience should exist");
    assert!(
        !experience.skills.contains(skill_id),
        "Experience should not reference the skill"
    );
}
