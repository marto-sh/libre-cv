Feature: Publish Professional Identity

  As an Owner,
  I want to publish or unpublish my Professional Identity,
  So that I can control whether my profile is publicly visible.

  Scenario: Owner publishes a drafted Professional Identity
    Given a Professional Identity has been drafted
    When the Owner publishes the Professional Identity
    Then the Professional Identity status should be "Published"
    And the Professional Identity should have a published datetime

  Scenario: Owner unpublishes a published Professional Identity
    Given a Professional Identity has been drafted
    And the Owner has published the Professional Identity
    When the Owner unpublishes the Professional Identity
    Then the Professional Identity status should be "Draft"
    And the Professional Identity should have no published datetime
