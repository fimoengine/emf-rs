# Contributing Guidelines

Thanks for taking the time to contribute!

The following is a set of guidelines for contributing to `emf-rs`. 
These are just guidelines, not rules, so use your best judgement and feel free 
to propose changes to this document in a pull request.

## Table of Contents

[table-of-contents]: #table-of-contents

- [Code of Conduct](#code-of-conduct)
- [Project Structure](#project-structure)
- [How Can I Contribute](#how-can-i-contribute)
  - [Issue](#issue)
  - [Feature Requests](#feature-requests)
  - [Pull Requests](#pull-requests)
- [Code Quality](#code-quality)

## Code of Conduct

[code-of-conduct]: #code-of-conduct

This project and everyone participating in it is governed by the 
[Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Project Structure

[project-structure]: #project-structure

The project is a monolithic repository containing multiple crates. 
The crates can be split up into low-level ffi-crates and idiomatic Rust crates.
The ffi crates are a direct port of the C libraries specified in the [EMF](https://github.com/fimoengine/emf)
and [RFC](https://github.com/fimoengine/emf-rfcs) repositories. 
The idiomatic crates are a slim wrapper over the ffi-crates and provide a more idiomatic API.
The ffi-crates are located in the [ffi directory](ffi).

## How Can I Contribute

[how-can-i-contribute]: #how-can-i-contribute

### Issue

[issue]: #issue

Ensure the bug was not already reported by searching on GitHub under 
[issues](https://github.com/fimoengine/emf-rs/issues). If you're unable to find an open issue addressing the bug, 
open a new [issue](https://github.com/fimoengine/emf-rs/issues/new/choose).

### Feature Requests

[feature-requests]: #feature-requests

You are welcome to submit a new Feature Request. Please use the appropriate issue template. 
Substantial changes to a ffi-crate require a new RFC to be accepted.

### Pull Requests

[pull-requests]: #pull-requests

Pull Requests are always welcome.

- Ensure that only ff-merges and rebases are used in your Git history.
- Clearly describe the problem and solution.
- Cleanup your commits with `git rebase`.

## Code Quality

[code-quality]: #code-quality

Rust offers some excellent tools to improve the quality of the codebase.
Automated tests will ensure proper code formatting and other quality standards.

- Use rustfmt
    - E.g `cargo fmt`
- Use clippy
    - E.g `cargo clippy`

