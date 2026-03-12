use cucumber::{given, then, when};

use crate::ProfessionalIdentityWorld;
use libre_cv::domain::digital_twin::aggregate::DigitalTwin;
use libre_cv::domain::professional_identity::aggregate::ProfessionalIdentity;
use libre_cv::domain::professional_identity::value_objects::Name;

#[given("a Digital Twin has been created")]
fn digital_twin_created(world: &mut ProfessionalIdentityWorld) {
    let name = Name::new("Ada Lovelace").expect("valid name");
    world.identity = Some(ProfessionalIdentity::draft(name));
    let id = world.identity.as_ref().unwrap().id().clone();
    world.digital_twin = Some(DigitalTwin::create(id));
}

#[when("the Owner creates a Digital Twin for the Professional Identity")]
fn create_digital_twin(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("Professional Identity should exist");
    let id = identity.id().clone();
    world.digital_twin = Some(DigitalTwin::create(id));
}

#[given(expr = "the Owner has set the tone to {string}")]
fn given_tone_set(world: &mut ProfessionalIdentityWorld, instruction: String) {
    let twin = world.digital_twin.as_mut().expect("Digital Twin should exist");
    twin.set_tone(&instruction).expect("should set tone");
}

#[when(expr = "the Owner sets the tone to {string}")]
fn set_tone(world: &mut ProfessionalIdentityWorld, instruction: String) {
    let twin = world.digital_twin.as_mut().expect("Digital Twin should exist");
    match twin.set_tone(&instruction) {
        Ok(()) => {}
        Err(e) => world.last_error = Some(e.to_string()),
    }
}

#[then("a Digital Twin should exist")]
fn digital_twin_should_exist(world: &mut ProfessionalIdentityWorld) {
    assert!(world.digital_twin.is_some(), "Digital Twin should exist");
}

#[then("the Digital Twin should have a tone")]
fn digital_twin_should_have_tone(world: &mut ProfessionalIdentityWorld) {
    let twin = world.digital_twin.as_ref().expect("Digital Twin should exist");
    assert!(twin.tone().is_some(), "Digital Twin should have a tone");
}

#[when("the Owner clears the tone")]
fn clear_tone(world: &mut ProfessionalIdentityWorld) {
    let twin = world.digital_twin.as_mut().expect("Digital Twin should exist");
    twin.clear_tone();
}

#[then(expr = "the tone should be {string}")]
fn tone_should_be(world: &mut ProfessionalIdentityWorld, expected: String) {
    let twin = world.digital_twin.as_ref().expect("Digital Twin should exist");
    let tone = twin.tone().expect("Digital Twin should have a tone");
    assert_eq!(tone.as_str(), expected);
}

#[then("the Digital Twin should have no tone")]
fn digital_twin_should_have_no_tone(world: &mut ProfessionalIdentityWorld) {
    let twin = world.digital_twin.as_ref().expect("Digital Twin should exist");
    assert!(twin.tone().is_none(), "Digital Twin should have no tone");
}
