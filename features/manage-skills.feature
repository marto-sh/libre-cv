Feature: Manage Skills on a Professional Identity

  As an Owner,
  I want to manage skills on my Professional Identity,
  So that my Digital Twin can share my capabilities.

  # --- Add ---

  Scenario: Owner adds a skill
    Given a Professional Identity has been drafted
    When the Owner adds a skill named "Rust"
    Then the Professional Identity should have 1 skill
    And the skill should have the name "Rust"

  Scenario: Owner adds a skill without a name
    Given a Professional Identity has been drafted
    When the Owner adds a skill named ""
    Then the skill should not be added

  # --- Update ---

  Scenario: Owner updates the name of a skill
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    When the Owner updates the skill name to "Rust Programming"
    Then the skill should have the name "Rust Programming"

  # --- Remove ---

  Scenario: Owner removes a skill
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    When the Owner removes the skill
    Then the Professional Identity should have 0 skills

  # --- Details ---

  Scenario: Owner adds a detail to a skill
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    When the Owner adds a detail "5 years of production experience" to the skill
    Then the skill should have 1 detail
    And the skill detail should have the text "5 years of production experience"

  Scenario: Owner updates a detail on a skill
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail "5 years of experience" to the skill
    When the Owner updates the skill detail text to "7 years of production experience"
    Then the skill detail should have the text "7 years of production experience"

  Scenario: Owner removes a detail from a skill
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail "5 years of experience" to the skill
    When the Owner removes the detail from the skill
    Then the skill should have 0 details

  # --- Cross-references with Experiences ---

  Scenario: Owner links a skill to an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    And the Owner has added a skill named "Rust"
    When the Owner links the skill to the experience
    Then the skill should reference the experience
    And the experience should reference the skill

  Scenario: Owner unlinks a skill from an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    And the Owner has added a skill named "Rust"
    And the Owner has linked the skill to the experience
    When the Owner unlinks the skill from the experience
    Then the skill should not reference the experience
    And the experience should not reference the skill
