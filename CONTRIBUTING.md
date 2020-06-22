# Contributing to propfuzz

We welcome contributions to `propfuzz` in the form of pull requests, bug reports
and feature suggestions.

## Pull Requests

We actively welcome your pull requests.

If you'd like to contribute a new feature, please file an RFC issue to discuss it
beforehand to ensure it will be accepted.

To create a new pull request:

1. Fork the repo and create your branch from `trunk`.
2. If you've added code that should be tested (i.e. not just a refactor), add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes with `cargo test --all-features`.
5. Run `cargo fmt` to automatically format your changes (CI will let you know if you missed this).
6. If you haven't already, complete the Contributor License Agreement ("CLA").

## Contributor License Agreement ("CLA")

In order to accept your pull request, we need you to submit a CLA. You only need
to do this once to work on any of Facebook's open source projects.

Complete your CLA here: <https://code.facebook.com/cla>

## Issues
We use GitHub issues to track public bugs. Please ensure your description is
clear and has sufficient instructions to be able to reproduce the issue.

Facebook has a [bounty program](https://www.facebook.com/whitehat/) for the safe
disclosure of security bugs. In those cases, please go through the process
outlined on that page and do not file a public issue.

## License

By contributing to `propfuzz`, you agree that your contributions will be dual-licensed under the terms of the
[`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) files in the root directory of this source
tree.
