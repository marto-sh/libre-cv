Feature: Manage Projects on a Professional Identity

  As an Owner,
  I want to manage projects on my Professional Identity,
  So that my Digital Twin can share my project portfolio.

  # --- Add ---

  Scenario: Owner adds a standalone project
    Given a Professional Identity has been drafted
    When the Owner adds a project named "Open Source CLI"
    Then the Professional Identity should have 1 project
    And the project should have the name "Open Source CLI"

  Scenario: Owner adds a project linked to an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    When the Owner adds a project named "Internal Dashboard" linked to the experience
    Then the Professional Identity should have 1 project
    And the project should have the name "Internal Dashboard"
    And the project should be linked to the experience

  Scenario: Owner adds a project without a name
    Given a Professional Identity has been drafted
    When the Owner adds a project named ""
    Then the project should not be added

  # --- Update ---

  Scenario: Owner updates the name of a project
    Given a Professional Identity has been drafted
    And the Owner has added a project named "CLI Tool"
    When the Owner updates the project name to "CLI Framework"
    Then the project should have the name "CLI Framework"

  # --- Remove ---

  Scenario: Owner removes a project
    Given a Professional Identity has been drafted
    And the Owner has added a project named "CLI Tool"
    When the Owner removes the project
    Then the Professional Identity should have 0 projects

  # --- Details ---

  Scenario: Owner adds a detail to a project
    Given a Professional Identity has been drafted
    And the Owner has added a project named "CLI Tool"
    When the Owner adds a detail titled "Tech stack" with text "Built with Rust and Clap" to the project
    Then the project should have 1 detail
    And the project detail should have the title "Tech stack"
    And the project detail should have the text "Built with Rust and Clap"

  Scenario: Owner updates a detail on a project
    Given a Professional Identity has been drafted
    And the Owner has added a project named "CLI Tool"
    And the Owner has added a detail titled "Tech stack" with text "Built with Rust" to the project
    When the Owner updates the project detail to title "Full stack" and text "Built with Rust and Clap"
    Then the project detail should have the title "Full stack"
    And the project detail should have the text "Built with Rust and Clap"

  Scenario: Owner removes a detail from a project
    Given a Professional Identity has been drafted
    And the Owner has added a project named "CLI Tool"
    And the Owner has added a detail titled "Tech stack" with text "Built with Rust" to the project
    When the Owner removes the detail from the project
    Then the project should have 0 details

  # --- Cross-references with Skills ---

  Scenario: Owner links a skill to a project
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a project named "CLI Tool"
    When the Owner links the skill to the project
    Then the skill should reference the project
    And the project should reference the skill
