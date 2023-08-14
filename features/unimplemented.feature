@unimplemented
Feature: Failing skipped steps

  Scenario: fail on skipped
    When I start with 2 items
    Then I fail this step
