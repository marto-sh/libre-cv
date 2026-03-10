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

  Scenario: Owner sets a summary
    When the Owner sets the summary to "Visionary analyst who foresaw the potential of computing beyond mere calculation."
    Then the summary should be "Visionary analyst who foresaw the potential of computing beyond mere calculation."

  Scenario: Owner updates the summary
    Given the summary is "Old summary."
    When the Owner sets the summary to "Updated summary."
    Then the summary should be "Updated summary."

  Scenario: Owner clears the summary
    Given the summary is "Some summary."
    When the Owner clears the summary
    Then the summary should be empty

  Scenario: Setting an empty headline is rejected
    When the Owner sets the headline to ""
    Then the operation should fail with "headline must not be empty"

  Scenario: Setting a whitespace-only headline is rejected
    When the Owner sets the headline to "   "
    Then the operation should fail with "headline must not be empty"

  Scenario: Setting an empty summary is rejected
    When the Owner sets the summary to ""
    Then the operation should fail with "summary must not be empty"

  Scenario: Setting a whitespace-only summary is rejected
    When the Owner sets the summary to "   "
    Then the operation should fail with "summary must not be empty"
