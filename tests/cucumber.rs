mod steps;

use cucumber::World;
use libre_cv::domain::professional_identity::aggregate::ProfessionalIdentity;
use libre_cv::domain::professional_identity::value_objects::{
    DetailId, ExpectationId, ExperienceId, ProjectId, SkillId,
};

#[derive(Debug, Default, World)]
pub struct ProfessionalIdentityWorld {
    identity: Option<ProfessionalIdentity>,
    current_experience_id: Option<ExperienceId>,
    current_project_id: Option<ProjectId>,
    current_skill_id: Option<SkillId>,
    current_expectation_id: Option<ExpectationId>,
    current_detail_id: Option<DetailId>,
    last_error: Option<String>,
}

#[tokio::main]
async fn main() {
    ProfessionalIdentityWorld::run("features/").await;
}
