Feature: Manage Headline and Summary

  As an Owner,
  I want to set and clear a headline and summary on my Professional Identity,
  So that visitors get a concise overview of who I am professionally.

  Background:
    Given a Professional Identity for "Ada Lovelace"

  Scenario: Owner sets a headline
    When the Owner sets the headline to "Pioneering Mathematician & First Computer Programmer"
    Then the headline should be "Pioneering Mathematician & First Computer Programmer"

  Scenario: Owner updates the headline
    Given the headline is "Old Headline"
    When the Owner sets the headline to "Updated Headline"
    Then the headline should be "Updated Headline"

  Scenario: Owner clears the headline
    Given the headline is "Some Headline"
    When the Owner clears the headline
    Then the headline should be empty

  Scenario: Setting an empty headline is rejected
    When the Owner sets the headline to ""
    Then the operation should fail with "headline must not be empty"

  Scenario: Setting a whitespace-only headline is rejected
    When the Owner sets the headline to "   "
    Then the operation should fail with "headline must not be empty"
