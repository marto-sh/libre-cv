use cucumber::{given, then, when};
use libre_cv::domain::professional_identity::aggregate::ProfessionalIdentity;
use libre_cv::domain::professional_identity::value_objects::Name;

use crate::ProfessionalIdentityWorld;

#[given(expr = "a Professional Identity for {string}")]
fn a_professional_identity(world: &mut ProfessionalIdentityWorld, name: String) {
    let name = Name::new(&name).expect("valid name");
    world.identity = Some(ProfessionalIdentity::draft(name));
}

#[given(expr = "the headline is {string}")]
fn headline_is(world: &mut ProfessionalIdentityWorld, headline: String) {
    let identity = world.identity.as_mut().expect("identity exists");
    identity.set_headline(&headline).expect("valid headline");
}

#[when(expr = "the Owner sets the headline to {string}")]
fn set_headline(world: &mut ProfessionalIdentityWorld, headline: String) {
    let identity = world.identity.as_mut().expect("identity exists");
    match identity.set_headline(&headline) {
        Ok(()) => world.last_error = None,
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[when("the Owner clears the headline")]
fn clear_headline(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_mut().expect("identity exists");
    identity.clear_headline();
}

#[then(expr = "the headline should be {string}")]
fn headline_should_be(world: &mut ProfessionalIdentityWorld, expected: String) {
    let identity = world.identity.as_ref().expect("identity exists");
    assert_eq!(
        identity.headline(),
        Some(expected.as_str()),
        "headline mismatch"
    );
}

#[then("the headline should be empty")]
fn headline_should_be_empty(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("identity exists");
    assert_eq!(identity.headline(), None, "headline should be empty");
}

#[then(expr = "the operation should fail with {string}")]
fn operation_should_fail(world: &mut ProfessionalIdentityWorld, expected_message: String) {
    let error = world
        .last_error
        .as_ref()
        .expect("expected an error but none occurred");
    assert!(
        error.contains(&expected_message),
        "expected error containing '{}', got '{}'",
        expected_message,
        error
    );
}
