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

  # --- Update ---

  Scenario: Owner updates the name of an expectation
    Given a Professional Identity has been drafted
    And the Owner has added a constraint named "No defense sector"
    When the Owner updates the expectation name to "No military or defense"
    Then the expectation should have the name "No military or defense"

  Scenario: Owner changes a constraint to a preference
    Given a Professional Identity has been drafted
    And the Owner has added a constraint named "Remote work"
    When the Owner changes the expectation kind to Preference
    Then the expectation should be a Preference

  # --- Remove ---

  Scenario: Owner removes an expectation
    Given a Professional Identity has been drafted
    And the Owner has added a constraint named "No defense sector"
    When the Owner removes the expectation
    Then the Professional Identity should have 0 expectations

  # --- Details ---

  Scenario: Owner adds a detail to an expectation
    Given a Professional Identity has been drafted
    And the Owner has added a preference named "Remote work"
    When the Owner adds a detail titled "Proven track record" with text "3 years of successful remote collaboration" to the expectation
    Then the expectation should have 1 detail
    And the expectation detail should have the title "Proven track record"
    And the expectation detail should have the text "3 years of successful remote collaboration"

  Scenario: Owner adds an expectation without a name
    Given a Professional Identity has been drafted
    When the Owner adds a constraint named ""
    Then the expectation should not be added
