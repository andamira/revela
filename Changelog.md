# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- new features: `unsafe_init`, `unsafe_libc`, `unsafe_constructors`.

### Changed
- rename example `alloc` to `no_std_alloc`.
- rename example `no_alloc` to `no_std_no_alloc_libc`.
- deprecate and rename feature `backends_all` to `all_std`.
- deprecate and rename feature `backends_alloc` → `all_alloc`.
- deprecate and rename feature `backends_no_std` → `all_no_std`.
- change `not(feature = "safe")` for `feature = "unsafe_*"`
- separate lengthy module level docs into markdown files.

### Fixed
- improve features and safety documentation.

## [0.0.8] - 2022-08-30

## [0.0.7] - 2022-04-11

## [0.0.6] - 2022-03-16

## [0.0.5] - 2022-03-16

## [0.0.4] - 2022-03-13

## [0.0.3] - 2022-02-19

## [0.0.2] - 2022-02-17

## [0.0.1] - 2023-02-09

[unreleased]: https://github.com/andamira/revela/compare/v0.0.8...HEAD
[0.0.8]: https://github.com/andamira/revela/releases/tag/v0.0.8
[0.0.7]: https://github.com/andamira/revela/releases/tag/v0.0.7
[0.0.6]: https://github.com/andamira/revela/releases/tag/v0.0.6
[0.0.5]: https://github.com/andamira/revela/releases/tag/v0.0.5
[0.0.4]: https://github.com/andamira/revela/releases/tag/v0.0.4
[0.0.3]: https://github.com/andamira/revela/releases/tag/v0.0.3
[0.0.2]: https://github.com/andamira/revela/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/revela/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
