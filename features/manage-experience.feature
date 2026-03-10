Feature: Manage Experiences on a Professional Identity

  As an Owner,
  I want to manage professional experiences on my Professional Identity,
  So that my Digital Twin can share my professional background.

  # --- Add ---

  Scenario: Owner adds an experience with a role
    Given a Professional Identity has been drafted
    When the Owner adds an experience with role "Software Engineer"
    Then the Professional Identity should have 1 experience
    And the experience should have the role "Software Engineer"

  Scenario: Owner adds an experience with a role and organization
    Given a Professional Identity has been drafted
    When the Owner adds an experience with role "Software Engineer" at "Acme Corp"
    Then the experience should have the role "Software Engineer"
    And the experience should have the organization "Acme Corp"

  Scenario: Owner adds an experience without a role
    Given a Professional Identity has been drafted
    When the Owner adds an experience with role ""
    Then the experience should not be added

  # --- Update ---

  Scenario: Owner updates the role of an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Junior Developer" at "Acme Corp"
    When the Owner updates the experience role to "Senior Developer"
    Then the experience should have the role "Senior Developer"

  Scenario: Owner updates the organization of an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    When the Owner updates the experience organization to "Globex Inc"
    Then the experience should have the organization "Globex Inc"

  # --- Remove ---

  Scenario: Owner removes an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    When the Owner removes the experience
    Then the Professional Identity should have 0 experiences

  # --- Details ---

  Scenario: Owner adds a detail to an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    When the Owner adds a detail "Led migration to Kubernetes" to the experience
    Then the experience should have 1 detail
    And the detail should have the text "Led migration to Kubernetes"

  Scenario: Owner adds multiple details to an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    When the Owner adds a detail "Led migration to Kubernetes" to the experience
    And the Owner adds a detail "Reduced deployment time by 40%" to the experience
    Then the experience should have 2 details

  Scenario: Owner updates a detail on an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    And the Owner has added a detail "Led migration to Kubernetes" to the experience
    When the Owner updates the detail text to "Led migration to Kubernetes across 3 regions"
    Then the detail should have the text "Led migration to Kubernetes across 3 regions"

  Scenario: Owner removes a detail from an experience
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Software Engineer" at "Acme Corp"
    And the Owner has added a detail "Led migration to Kubernetes" to the experience
    When the Owner removes the detail from the experience
    Then the experience should have 0 details
