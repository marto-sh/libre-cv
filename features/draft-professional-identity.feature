Feature: Draft Professional Identity

  As an Owner,
  I want to draft my Professional Identity,
  So that I can start building my digital professional representation.

  Scenario: Owner drafts a Professional Identity with their name
    Given an Owner wants to create their Professional Identity
    When the Owner drafts a Professional Identity with name "Ada Lovelace"
    Then a Professional Identity should be drafted
    And the Professional Identity should have the name "Ada Lovelace"
