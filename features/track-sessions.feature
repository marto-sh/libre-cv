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

  # --- Idempotency ---

  Scenario: Adding the same session twice is idempotent
    Given a Professional Identity has been drafted
    When the Owner adds a session "session-001"
    And the Owner adds a session "session-001"
    Then the Professional Identity should have 1 session
