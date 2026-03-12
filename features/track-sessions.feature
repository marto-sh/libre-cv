Feature: Track Sessions on a Professional Identity

  As an Owner,
  I want to track which sessions contributed to my Professional Identity,
  So that my Digital Twin can trace information back to its source.

  # --- Add session ---

  Scenario: Owner adds a session
    Given a Professional Identity has been drafted
    When the Owner adds a session "session-001"
    Then the Professional Identity should have 1 session
