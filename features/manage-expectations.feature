Feature: Manage Expectations on a Professional Identity

  As an Owner,
  I want to manage expectations on my Professional Identity,
  So that my Digital Twin can share what I expect from opportunities.

  # --- Add ---

  Scenario: Owner adds a constraint
    Given a Professional Identity has been drafted
    When the Owner adds a constraint named "No defense sector"
    Then the Professional Identity should have 1 expectation
    And the expectation should have the name "No defense sector"
    And the expectation should be a Constraint

  Scenario: Owner adds a preference
    Given a Professional Identity has been drafted
    When the Owner adds a preference named "Remote work"
    Then the Professional Identity should have 1 expectation
    And the expectation should have the name "Remote work"
    And the expectation should be a Preference

  Scenario: Owner adds an expectation without a name
    Given a Professional Identity has been drafted
    When the Owner adds a constraint named ""
    Then the expectation should not be added
