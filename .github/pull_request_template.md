## Description

Brief description of the changes.

## Type of change

- [ ] Performance improvement
- [ ] Bug fix
- [ ] Other

## Checklist

- [ ] Pull request title and commit messages are clear and informative.
- [ ] Documentation has been updated if necessary.
- [ ] Code style matches the existing code. This one is somewhat subjective, but try to "fit in" by
  using the same naming conventions. Code should be portable, avoiding any architecture
  specific intrinsics.
- [ ] Tests pass `cargo test`
- [ ] Code is formatted ``cargo fmt -- `find . -name "*.rs"` ``
- [ ] Code is linted `cargo clippy --all-targets --all-features`

Formatting and linting also can be executed by running [`just`](https://github.com/casey/just)
(if installed) on the command line at the project root.
