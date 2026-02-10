# run all checks, formatting, linting, etc.
check: fmt lint
    @cargo check --features client -q

# debug build of the library
build-debug:
    @cargo build --debug --features client

# format code
fmt:
    @cargo fmt

# run clippy lints
lint:
    @cargo clippy

# clean artifacts
clean:
    @cargo clean

# run all test cases
test:
    @cargo test -q --features client
