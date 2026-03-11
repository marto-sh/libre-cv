use cucumber::{given, then, when};

use crate::ProfessionalIdentityWorld;

// --- Add ---

#[when(expr = "the Owner adds a project named {string}")]
fn add_project(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    match identity.add_project(&name, None) {
        Ok(id) => world.current_project_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when(expr = "the Owner adds a project named {string} linked to the experience")]
fn add_project_linked(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    match identity.add_project(&name, Some(exp_id)) {
        Ok(id) => world.current_project_id = Some(id),
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[given(expr = "the Owner has added a project named {string}")]
fn given_project_added(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = identity
        .add_project(&name, None)
        .expect("should add project");
    world.current_project_id = Some(id);
}

// --- Update ---

#[when(expr = "the Owner updates the project name to {string}")]
fn update_project_name(world: &mut ProfessionalIdentityWorld, name: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    identity
        .update_project_name(id, &name)
        .expect("should update project name");
}

// --- Details ---

#[when(expr = "the Owner adds a detail titled {string} with text {string} to the project")]
fn add_detail_to_project(world: &mut ProfessionalIdentityWorld, title: String, text: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = identity
        .add_detail_to_project(id, &title, &text)
        .expect("should add detail");
    world.current_detail_id = Some(detail_id);
}

#[given(expr = "the Owner has added a detail titled {string} with text {string} to the project")]
fn given_detail_added_to_project(world: &mut ProfessionalIdentityWorld, title: String, text: String) {
    add_detail_to_project(world, title, text);
}

#[when(expr = "the Owner updates the project detail to title {string} and text {string}")]
fn update_project_detail(world: &mut ProfessionalIdentityWorld, title: String, text: String) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = world
        .current_detail_id
        .as_ref()
        .expect("should have a current detail");
    identity
        .update_detail_on_project(id, detail_id, &title, &text)
        .expect("should update detail");
}

#[when("the Owner removes the detail from the project")]
fn remove_detail_from_project(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let detail_id = world
        .current_detail_id
        .as_ref()
        .expect("should have a current detail");
    identity
        .remove_detail_from_project(id, detail_id)
        .expect("should remove detail");
}

// --- Cross-references ---

#[when("the Owner links the skill to the project")]
fn link_skill_to_project(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    identity
        .link_skill_to_project(skill_id, project_id)
        .expect("should link");
}

#[given("the Owner has linked the skill to the project")]
fn given_skill_linked_to_project(world: &mut ProfessionalIdentityWorld) {
    link_skill_to_project(world);
}

#[when("the Owner unlinks the skill from the project")]
fn unlink_skill_from_project(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    identity
        .unlink_skill_from_project(skill_id, project_id)
        .expect("should unlink");
}

// --- Remove ---

#[when("the Owner removes the project")]
fn remove_project(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity should exist");
    let id = world
        .current_project_id
        .take()
        .expect("should have a current project");
    identity.remove_project(&id).expect("should remove project");
}

// --- Assertions ---

#[then(expr = "the Professional Identity should have {int} project(s)")]
fn should_have_n_projects(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.projects().len(), count);
}

#[then(expr = "the project should have the name {string}")]
fn project_should_have_name(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let project = identity.project(id).expect("project should exist");
    assert_eq!(project.name, expected);
}

#[then("the project should be linked to the experience")]
fn project_should_be_linked_to_experience(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let exp_id = world
        .current_experience_id
        .as_ref()
        .expect("should have a current experience");
    let project = identity.project(project_id).expect("project should exist");
    assert_eq!(
        project.experience.as_ref(),
        Some(exp_id),
        "Project should be linked to the experience"
    );
}

#[then(expr = "the project should have {int} detail(s)")]
fn project_should_have_n_details(world: &mut ProfessionalIdentityWorld, count: usize) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let project = identity.project(id).expect("project should exist");
    assert_eq!(project.details.len(), count);
}

#[then(expr = "the project detail should have the title {string}")]
fn project_detail_should_have_title(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let project = identity.project(id).expect("project should exist");
    let detail = project.details.first().expect("should have a detail");
    assert_eq!(detail.title, expected);
}

#[then(expr = "the project detail should have the text {string}")]
fn project_detail_should_have_text(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let project = identity.project(id).expect("project should exist");
    let detail = project.details.first().expect("should have a detail");
    assert_eq!(detail.text, expected);
}

#[then("the skill should reference the project")]
fn skill_should_reference_project(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let skill = identity.skill(skill_id).expect("skill should exist");
    assert!(
        skill.projects.contains(project_id),
        "Skill should reference the project"
    );
}

#[then("the project should reference the skill")]
fn project_should_reference_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let project = identity.project(project_id).expect("project should exist");
    assert!(
        project.skills.contains(skill_id),
        "Project should reference the skill"
    );
}

#[then("the skill should not reference the project")]
fn skill_should_not_reference_project(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let skill = identity.skill(skill_id).expect("skill should exist");
    assert!(
        !skill.projects.contains(project_id),
        "Skill should not reference the project"
    );
}

#[then("the project should not reference the skill")]
fn project_should_not_reference_skill(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity should exist");
    let skill_id = world
        .current_skill_id
        .as_ref()
        .expect("should have a current skill");
    let project_id = world
        .current_project_id
        .as_ref()
        .expect("should have a current project");
    let project = identity.project(project_id).expect("project should exist");
    assert!(
        !project.skills.contains(skill_id),
        "Project should not reference the skill"
    );
}

#[then("the project should not be added")]
fn project_should_not_be_added(world: &mut ProfessionalIdentityWorld) {
    assert!(
        world.last_error.is_some(),
        "Expected an error when adding project"
    );
    let identity = world.identity.as_ref().expect("identity should exist");
    assert_eq!(identity.projects().len(), 0);
}
