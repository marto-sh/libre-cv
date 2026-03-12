Feature: Configure Digital Twin

  As an Owner,
  I want to configure my Digital Twin,
  So that it can act on my behalf according to my preferences.

  Scenario: Owner creates a Digital Twin for their Professional Identity
    Given a Professional Identity has been drafted
    When the Owner creates a Digital Twin for the Professional Identity
    Then a Digital Twin should exist
