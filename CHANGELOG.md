# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This is the changelog for the Rust port of the reporters-db library. This port maintains 100% API compatibility with the original Python version while providing significant performance improvements through compile-time optimization.

## [Unreleased]

### Added
- Automatic README.md version updates during release process via cargo-release
- Pre-release replacement configuration to keep documentation in sync with published version

## [0.1.4] - 2025-09-13

### Infrastructure
- Improved GitHub Actions release workflow security
- Added 'release' environment to publish job for better secret management
- Changed cargo registry token to use environment variable instead of CLI flag

## [0.1.3] - 2025-09-13

### Fixed
- GitHub Actions workflow validation errors using invalid hashFiles() function
- Replaced unsupported hashFiles() with explicit if: false conditions
- All Python workflows now properly skip in Rust fork

## [0.1.2] - 2025-09-13

### Fixed
- All clippy warnings resolved using modern Rust patterns
- Collapsible if statements now use let-else chains
- Removed needless reference comparisons
- Improved iterator usage with `.values()` method
- Enhanced code quality and maintainability

## [0.1.1] - 2025-09-13

### Infrastructure
- Disabled Python workflows for Rust fork while preserving merge compatibility
- Added conditional checks to automatically detect Rust vs Python repository
- Improved CI/CD pipeline reliability

## [0.1.0] - 2025-09-13

### Added
- **Complete PHF Modernization**: Zero runtime overhead through compile-time Perfect Hash Function data structures
- **Comprehensive Test Suite**: 28 unit tests covering data integrity and validation
- **Modern Rust 2024 Edition**: Latest language features and best practices
- Modern GitHub Actions release workflow with automated publishing to crates.io
- `cargo-release` integration with automated changelog updates
- Comprehensive consolidated changelog merging Rust-specific and upstream changes

### Architecture
- **PHF Optimization**: All JSON data embedded as compile-time perfect hash maps
- **Shared Types Module**: Clean separation between build-time parsing and runtime access
- **Generated Code**: Static reference types (`&'static str`) for zero-copy access
- **Wildcard Import Linting**: Enforced explicit imports for better code clarity

### Performance
- **Build time**: Data processed during compilation, not runtime
- **Memory usage**: Static data structures, no dynamic allocation
- **Access time**: O(1) hash lookups with perfect hashing
- **Binary size**: Optimized with compile-time code generation

### Infrastructure
- **Release Automation**: Complete CI/CD pipeline for automated releases
- **Changelog Management**: Automated version updates and changelog generation
- **Quality Gates**: Formatting, clippy, and test validation before release
- **Publishing**: Automated crates.io publishing on successful releases

### Documentation
- Enhanced changelog with detailed architecture and performance documentation
- Modern markdown README with improved project description
- Comprehensive API compatibility documentation
- Architecture Decision Records (ADRs) documenting PHF migration rationale

## [0.0.5] - 2024-12-13

### Fixed
- Resolved rebase conflicts and compatibility issues with upstream data
- Fixed name collisions in build.rs code generation with counter system
- Updated validation to handle Unicode quotes in legal publication names
- All tests now pass (28 unit + 20 doc tests)

## [0.0.4] - Previous Release
### Fixed
- Access control improvements making utils public
- Basic Rust port with runtime JSON parsing
- Initial PHF exploration

## [0.0.3] - Previous Release
### Added
- Initial Rust port of the freelawproject reporters database
- Early Rust migration work
- JSON embedding experiments

## [0.0.2] - Previous Release
### Added
- Initial Rust structure setup

## [0.0.1] - Previous Release
### Added
- Project initialization

[Unreleased]: https://github.com/jakeswenson/reporters-db/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/jakeswenson/reporters-db/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jakeswenson/reporters-db/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jakeswenson/reporters-db/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jakeswenson/reporters-db/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jakeswenson/reporters-db/compare/v0.0.5...v0.1.0
[0.0.5]: https://github.com/jakeswenson/reporters-db/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/jakeswenson/reporters-db/releases/tag/v0.0.4
[0.0.3]: https://github.com/jakeswenson/reporters-db/releases/tag/v0.0.3
[0.0.2]: https://github.com/jakeswenson/reporters-db/releases/tag/v0.0.2
[0.0.1]: https://github.com/jakeswenson/reporters-db/releases/tag/v0.0.1