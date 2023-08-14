@slow
Feature: My slow test

  Scenario: counting items
    Given I start with 2 items
    And I pause for 1 second
    When I add 3 items
    Then I have 5 items
