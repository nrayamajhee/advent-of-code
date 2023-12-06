# Advent of Code 2020

## Build

```
cargo build
```

## Run

First make sure you're in the `crate directory` and not inside `src` or `test`, then run:

```
cargo run
```

This is because I read the inputs from `std::env::current_dir()/input`. If you run `cargo run` from `src` directory, it is going to try to read input from `crate/src/input`, which is not a valid directory.

## Test

```
cargo test
```

You could also run test from a particular day like so:

```
cargo test day1
```

The same working directory requirement as above applies for tests.

## View documentation

```
cargo doc --document-private-items --open
```
