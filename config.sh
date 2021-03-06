#! /usr/bin/env bash
export RUSTUP_HOME=/usr/local/rustup
export CARGO_HOME=/usr/local/cargo
export PATH=/usr/local/cargo/bin:$PATH

# these flags are necessary for grcov
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
 
# file structure
AUTOGRADER=/autograder/autograder/Cargo.toml
ASSIGNMENT=/autograder/assignment/Cargo.toml
SUBMISSION=/autograder/submission/Cargo.toml
OUR_TEST_OUTPUTS=/autograder/our_test_outputs
THEIR_TEST_OUTPUTS=/autograder/their_test_outputs
OUTPUT=/autograder/results/results.json
LCOV=/autograder/submission/ccov/lcov.info
SCORES=/autograder/scores.yaml
LABELS=/autograder/labels.yaml
OUR_SOLUTION=/autograder/assignment/src/solution.rs
THEIR_SOLUTION=/autograder/submission/src/solution.rs

