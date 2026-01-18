# run all checks, formatting, linting, etc.
check: fmt lint
    cargo check

# debug build of the library
build-debug:
    cargo build --debug

# format code
fmt:
    cargo fmt

# run clippy lints
lint:
    cargo clippy
