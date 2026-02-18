use cucumber::{given, then, when};

use crate::ProfessionalIdentityWorld;
use libre_cv::domain::professional_identity::aggregate::{ProfessionalIdentity, Status};
use libre_cv::domain::professional_identity::value_objects::Name;

#[given("a Professional Identity has been drafted")]
fn professional_identity_drafted(world: &mut ProfessionalIdentityWorld) {
    let name = Name::new("Ada Lovelace").expect("valid name");
    world.identity = Some(ProfessionalIdentity::draft(name));
}

#[given("the Owner has published the Professional Identity")]
fn owner_has_published(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("Professional Identity should exist");
    identity.publish();
}

#[when("the Owner publishes the Professional Identity")]
fn owner_publishes(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("Professional Identity should exist");
    identity.publish();
}

#[when("the Owner unpublishes the Professional Identity")]
fn owner_unpublishes(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("Professional Identity should exist");
    identity.unpublish();
}

#[then(expr = "the Professional Identity status should be {string}")]
fn identity_status_should_be(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("Professional Identity should exist");
    let expected_status = match expected.as_str() {
        "Published" => Status::Published,
        "Draft" => Status::Draft,
        other => panic!("Unknown status: {other}"),
    };
    assert_eq!(identity.status(), expected_status);
}

#[then("the Professional Identity should have a published datetime")]
fn identity_should_have_published_datetime(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("Professional Identity should exist");
    assert!(identity.published_at().is_some(), "Expected a published datetime");
}

#[then("the Professional Identity should have no published datetime")]
fn identity_should_have_no_published_datetime(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("Professional Identity should exist");
    assert!(identity.published_at().is_none(), "Expected no published datetime");
}
