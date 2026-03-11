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
