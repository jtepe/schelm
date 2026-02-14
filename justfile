# run all checks, formatting, linting, etc.
check: fmt lint
    @cargo check --features client -q

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
