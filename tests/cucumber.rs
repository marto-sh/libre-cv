mod steps;

use cucumber::World;
use libre_cv::domain::professional_identity::aggregate::ProfessionalIdentity;

#[derive(Debug, Default, World)]
pub struct ProfessionalIdentityWorld {
    identity: Option<ProfessionalIdentity>,
}

#[tokio::main]
async fn main() {
    ProfessionalIdentityWorld::run("features/").await;
}
