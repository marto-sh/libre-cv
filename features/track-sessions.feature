Feature: Track Sessions on a Professional Identity

  As an Owner,
  I want to track which sessions contributed to my Professional Identity,
  So that my Digital Twin can trace information back to its source.

  # --- Add session ---

  Scenario: Owner adds a session
    Given a Professional Identity has been drafted
    When the Owner adds a session "session-001"
    Then the Professional Identity should have 1 session

  Scenario: Owner adds a session with an empty id
    Given a Professional Identity has been drafted
    When the Owner adds a session ""
    Then the session should not be added

  # --- Remove session ---

  Scenario: Owner removes a session
    Given a Professional Identity has been drafted
    And the Owner has added a session "session-001"
    When the Owner removes the session "session-001"
    Then the Professional Identity should have 0 sessions

  Scenario: Owner removes a session that does not exist
    Given a Professional Identity has been drafted
    When the Owner removes the session "nonexistent"
    Then the session should not be removed

  # --- Add source to detail ---

  Scenario: Owner adds a source to a skill detail
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail titled "Systems programming" with text "Built low-level systems" to the skill
    When the Owner adds a source from session "session-001" turn "turn-42" to the skill detail
    Then the skill detail should have 1 source

  Scenario: Owner adds a source to an experience detail
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Engineer" at "Acme"
    And the Owner has added a detail titled "Led team" with text "Managed 5 engineers" to the experience
    When the Owner adds a source from session "session-001" turn "turn-42" to the experience detail
    Then the experience detail should have 1 source

  Scenario: Owner adds a source to a project detail
    Given a Professional Identity has been drafted
    And the Owner has added a project named "Widget"
    And the Owner has added a detail titled "Architecture" with text "Designed the system" to the project
    When the Owner adds a source from session "session-001" turn "turn-42" to the project detail
    Then the project detail should have 1 source

  Scenario: Owner adds a source to an expectation detail
    Given a Professional Identity has been drafted
    And the Owner has added a constraint named "No defense"
    And the Owner has added a detail titled "Ethical" with text "Personal values" to the expectation
    When the Owner adds a source from session "session-001" turn "turn-42" to the expectation detail
    Then the expectation detail should have 1 source

  # --- Remove source from detail ---

  Scenario: Owner removes a source from a skill detail
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail titled "Systems programming" with text "Built low-level systems" to the skill
    And the Owner has added a source from session "session-001" turn "turn-42" to the skill detail
    When the Owner removes the source from session "session-001" turn "turn-42" from the skill detail
    Then the skill detail should have 0 sources

  Scenario: Owner removes a source from an experience detail
    Given a Professional Identity has been drafted
    And the Owner has added an experience with role "Engineer" at "Acme"
    And the Owner has added a detail titled "Led team" with text "Managed 5 engineers" to the experience
    And the Owner has added a source from session "session-001" turn "turn-42" to the experience detail
    When the Owner removes the source from session "session-001" turn "turn-42" from the experience detail
    Then the experience detail should have 0 sources

  Scenario: Owner removes a source from a project detail
    Given a Professional Identity has been drafted
    And the Owner has added a project named "Widget"
    And the Owner has added a detail titled "Architecture" with text "Designed the system" to the project
    And the Owner has added a source from session "session-001" turn "turn-42" to the project detail
    When the Owner removes the source from session "session-001" turn "turn-42" from the project detail
    Then the project detail should have 0 sources

  Scenario: Owner removes a source from an expectation detail
    Given a Professional Identity has been drafted
    And the Owner has added a constraint named "No defense"
    And the Owner has added a detail titled "Ethical" with text "Personal values" to the expectation
    And the Owner has added a source from session "session-001" turn "turn-42" to the expectation detail
    When the Owner removes the source from session "session-001" turn "turn-42" from the expectation detail
    Then the expectation detail should have 0 sources

  Scenario: Owner removes a source that does not exist
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail titled "Systems programming" with text "Built low-level systems" to the skill
    When the Owner removes the source from session "nonexistent" turn "turn-42" from the skill detail
    Then the source should not be removed

  # --- Source validation ---

  Scenario: Owner adds a source with an empty session id
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail titled "Systems programming" with text "Built low-level systems" to the skill
    When the Owner tries to add a source from session "" turn "turn-42" to the skill detail
    Then the source should not be added

  Scenario: Owner adds a source with an empty turn id
    Given a Professional Identity has been drafted
    And the Owner has added a skill named "Rust"
    And the Owner has added a detail titled "Systems programming" with text "Built low-level systems" to the skill
    When the Owner tries to add a source from session "session-001" turn "" to the skill detail
    Then the source should not be added

  # --- Idempotency ---

  Scenario: Adding the same session twice is idempotent
    Given a Professional Identity has been drafted
    When the Owner adds a session "session-001"
    And the Owner adds a session "session-001"
    Then the Professional Identity should have 1 session
