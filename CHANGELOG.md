# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2024-12-12
- Add a public archive note and state the last supported rust version `1.71.0-nightly (nightly-2023-04-18)`.
- Removed the CI.

## [0.3.0] - 2022-11-22
- Reworked the crate to use only a single Struct `ConstClosure`.
- Allow borrowing tuples of up to 12 references (Currently only of the same mutability).
- Updated the docs for the new version.
- Breaking: Removed the old Types (`ConstFnClosure`, `ConstFnMutClosure`, `ConstFnOnceClosure`).
- Breaking: Require the Captured Data and Arguments of the implementation function to be Tuples for consistency.

## [0.2.3] - 2022-11-10

- Minor doc improvements.

### Fixes
- Fixed daily nightly ci badge.
- Added the new `Tuple` marker trait to trait bounds where necessary.

## [0.2.2] - 2022-09-25

- Moved to [feather-ink-org](https://github.com/ink-feather-org/const-closure-rs)

### Fixes
- Fixed clippy lints.

## [0.2.1] - 2022-09-21

### Fixes
- Fixed lifetime issues.

## [0.2.0] - 2022-09-18

### Added
- `ConstFnClosure`
- `ConstFnMutClosure`
- `ConstFnOnceClosure`

### Removed
- `const_closure` macro

## [0.1.0] - 2022-09-13

Initial release.

[Unreleased]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ink-feather-org/const-closure-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ink-feather-org/const-closure-rs/releases/tag/v0.1.0
