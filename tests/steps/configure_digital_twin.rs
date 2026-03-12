use cucumber::{then, when};

use crate::ProfessionalIdentityWorld;
use libre_cv::domain::digital_twin::aggregate::DigitalTwin;

#[when("the Owner creates a Digital Twin for the Professional Identity")]
fn create_digital_twin(world: &mut ProfessionalIdentityWorld) {
    let identity = world.identity.as_ref().expect("Professional Identity should exist");
    let id = identity.id().clone();
    world.digital_twin = Some(DigitalTwin::create(id));
}

#[then("a Digital Twin should exist")]
fn digital_twin_should_exist(world: &mut ProfessionalIdentityWorld) {
    assert!(world.digital_twin.is_some(), "Digital Twin should exist");
}
