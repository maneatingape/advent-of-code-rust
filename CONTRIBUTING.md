# Contributing

I hope that you've enjoyed reading these solutions as much as I enjoyed writing them.
They're pretty fast and clean...but can you make them even *faster and cleaner*?

If you've made an improvement, then please
[open a pull request](https://github.com/maneatingape/advent-of-code-rust/compare).
Contributions are encouraged and valued. 🎁

Your contribution should generally fall into one of the following two categories:

* 🚀 **Performance Improvement** Your pull request gives a significant performance
  improvement. This could be achieved by using a better algorithm, better low-level optimizations,
  by applying the portable SIMD library or by any other means!
* 🪲 **Bug Fix** Solutions try to be as general as possible but can't test for every input.
  Your pull request fixes an incorrect answer for your input or prevents a panic.

## Legal Notice

When contributing to this project, you must agree that you have authored 100% of the content,
that you have the necessary rights to the content and that the content you contribute
may be provided under the project license.

## Guidelines

Pull requests should meet the following baseline standards:

* Pull request title and commit messages are clear and informative.
* Documentation has been updated if necessary.
* Code style matches the existing code. This one is somewhat subjective, but try to "fit in" by
  using the same naming conventions. Code should be portable, avoiding any architecture
  specific intrinsics.
* Tests pass

      cargo test

* Code is formatted

      cargo fmt -- `find . -name "*.rs"`

* Code is linted

      cargo clippy --all-targets --all-features

Formatting and linting also can be executed by running [`just`](https://github.com/casey/just)
(if installed) on the command line at the project root.
