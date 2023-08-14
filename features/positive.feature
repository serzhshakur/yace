@positive
Feature: My test

  Scenario: counting items 1
    Given I start with 1 items
    When I add 3 items
    Then I have 4 items

  Scenario: counting items 2
    Given I start with 2 items
    Then I have 2 items

  Scenario: counting items 3
    Given I start with 3 items
    When I add 1 items
    Then I have 4 items

  Scenario: counting 4 items
    Given I start with 4 items
    When I add 1 items
    When I add 2 items
    When I add 3 items
    Then I have 10 items
