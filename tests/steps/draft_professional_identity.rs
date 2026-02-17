use cucumber::{given, then, when};
use libre_cv::domain::professional_identity::aggregate::ProfessionalIdentity;
use libre_cv::domain::professional_identity::value_objects::Name;

use crate::ProfessionalIdentityWorld;

#[given("an Owner wants to create their Professional Identity")]
fn owner_wants_to_create(_world: &mut ProfessionalIdentityWorld) {
    // Establishes intent — no-op
}

#[when(expr = "the Owner drafts a Professional Identity with name {string}")]
fn owner_drafts_identity(world: &mut ProfessionalIdentityWorld, name: String) {
    let name = Name::new(&name).expect("valid name");
    world.identity = Some(ProfessionalIdentity::draft(name));
}

#[then("a Professional Identity should be drafted")]
fn identity_should_be_drafted(world: &mut ProfessionalIdentityWorld) {
    assert!(world.identity.is_some(), "Professional Identity should exist");
}

#[then(expr = "the Professional Identity should have the name {string}")]
fn identity_should_have_name(world: &mut ProfessionalIdentityWorld, expected_name: String) {
    let identity = world.identity.as_ref().expect("Professional Identity should exist");
    assert_eq!(identity.name().as_str(), expected_name);
}
