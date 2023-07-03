# yace - Yet Another cucumber-rs Example

This repo was created to report issues to [cucumber-rs](https://github.com/cucumber-rs/cucumber)

## Running tests

In order to run Cucumber tests under `./features` execute the following command

```
cargo run
```
or with Cucumber flags 

```
cargo run -- --retry=1 --tags="not @flaky"
```
