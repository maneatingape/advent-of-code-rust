lint:
    cargo fmt -- `find . -name "*.rs"`
    cargo clippy --all-targets --all-features

docs:
    cargo doc --document-private-items --open
