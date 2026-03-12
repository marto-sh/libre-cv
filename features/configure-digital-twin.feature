Feature: Configure Digital Twin

  As an Owner,
  I want to configure my Digital Twin,
  So that it can act on my behalf according to my preferences.

  Scenario: Owner creates a Digital Twin for their Professional Identity
    Given a Professional Identity has been drafted
    When the Owner creates a Digital Twin for the Professional Identity
    Then a Digital Twin should exist

  Scenario: Owner sets a tone on the Digital Twin
    Given a Digital Twin has been created
    When the Owner sets the tone to "Be concise and technical. Avoid jargon when possible."
    Then the Digital Twin should have a tone
    And the tone should be "Be concise and technical. Avoid jargon when possible."

  Scenario: Owner clears the tone from the Digital Twin
    Given a Digital Twin has been created
    And the Owner has set the tone to "Be friendly and approachable."
    When the Owner clears the tone
    Then the Digital Twin should have no tone

  Scenario: Owner cannot set an empty tone
    Given a Digital Twin has been created
    When the Owner sets the tone to ""
    Then the operation should fail with "tone must not be empty"

  Scenario: Owner cannot set a whitespace-only tone
    Given a Digital Twin has been created
    When the Owner sets the tone to "   "
    Then the operation should fail with "tone must not be empty"
