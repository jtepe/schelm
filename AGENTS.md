# Project Description

AI Agent development tools for Rust. The project is separated into different packages (see [Crates]).
The purpose is to provide all the tools necessary to develop AI agents.

## Crates

* [./crates/schelm-ores] : OpenResponses Rust SDK

## Development Instructions

Always run these commands after editing code to ensure the code base stays clean and in a
good state.
* `cargo fmt --all` : apply formatting rules
* `cargo clippy --all-targets --features client` : lint code
* `cargo check -q --features client` : runs static checks
* `RUST_LOG=off cargo test -q --features client` : runs all test cases
