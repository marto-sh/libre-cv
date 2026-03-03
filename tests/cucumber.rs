mod steps;

use cucumber::World;
use libre_cv::domain::professional_identity::aggregate::ProfessionalIdentity;
use libre_cv::domain::professional_identity::value_objects::ExperienceId;

#[derive(Debug, Default, World)]
pub struct ProfessionalIdentityWorld {
    identity: Option<ProfessionalIdentity>,
    current_experience_id: Option<ExperienceId>,
    last_error: Option<String>,
}

#[tokio::main]
async fn main() {
    ProfessionalIdentityWorld::run("features/").await;
}
