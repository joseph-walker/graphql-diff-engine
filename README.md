# GQL Diff Engine

Automated regression tool to run a set of GraphQL queries against two different API deployments and diff the responses.

## Motivation

Because GraphQL can't be versioned, it is often risky to do major refactoring or core infrastructure changes - you run the risk of breaking some edge case resolver. You can use snapshot testing, but it can be inflexibile and verbose. This tool provides a convention-over-configuration way to run a regression test against two different API versions or deployments.

## Running

TBD

## Adding query arguments, headers, etc.

TBD

# Development

```
git clone https://github.com/joseph-walker/graphql-diff-engine.git
cd graphql-diff-engine
cargo run -- -h
```
