@positive
Feature: My test

  Background: My background
    Given I start with 10 items

  Scenario: counting items 1
    When I add 3 items
    Then I have 13 items

  Scenario: counting items 2
    Given I start with 2 items
    Then I have 2 items

  Scenario: counting items 3
    When I add 1 items
    Then I have 11 items

  Scenario: counting 4 items
    When I add 1 items
    When I add 2 items
    When I add 3 items
    Then I have 16 items

  Rule: Rule name
    Background: My rule background
      Given I start with 1 items

    Scenario: scenario within a rule
      When I add 1 items
      Then I have 2 items
