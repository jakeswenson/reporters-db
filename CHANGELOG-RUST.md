# Rust Port Changelog

This is the changelog for the Rust port of the reporters-db library. This port maintains 100% API compatibility with the original Python version while providing significant performance improvements through compile-time optimization.

## Current Version

### 0.0.5 (2025-01-XX) - Complete Migration & Architecture Modernization

**Major Features:**
- ✅ Complete Python-to-Rust migration with 100% API compatibility
- ✅ Zero runtime overhead through compile-time PHF (Perfect Hash Function) data structures
- ✅ Comprehensive test suite with 28 unit tests covering data integrity and validation
- ✅ Modern Rust 2024 edition with latest language features

**Architecture:**
- **PHF Optimization**: All JSON data embedded as compile-time perfect hash maps
- **Shared Types Module**: Clean separation between build-time parsing and runtime access
- **Generated Code**: Static reference types (`&'static str`) for zero-copy access
- **Wildcard Import Linting**: Enforced explicit imports for better code clarity

**API Compatibility:**
- `get_reporters()` - Main reporters database
- `get_variations_only()` - Variation mappings
- `get_editions()` - Edition mappings
- `get_names_to_editions()` - Reporter name to edition mappings
- `get_regex_variables()` - Processed regex templates
- `get_state_abbreviations()` - State abbreviation mappings
- `get_case_name_abbreviations()` - Case name abbreviations
- `get_journals()` - Journal database
- `get_laws()` - Laws database
- `get_special_formats()` - Special formatting rules

**Performance:**
- **Build time**: Data processed during compilation, not runtime
- **Memory usage**: Static data structures, no dynamic allocation
- **Access time**: O(1) hash lookups with perfect hashing
- **Binary size**: Optimized with compile-time code generation

**Quality Assurance:**
- 28 comprehensive unit tests (15 basic API + 13 advanced validation)
- Data integrity validation (ASCII compliance, whitespace, dates)
- Consistency validation (variations mapping, cite types, edition ordering)
- Coverage validation (database size checks, mapping completeness)

## Past Versions

### 0.0.4 (Previous)
- Basic Rust port with runtime JSON parsing
- Initial PHF exploration

### 0.0.3 (Previous)
- Early Rust migration work
- JSON embedding experiments

### 0.0.2 (Previous)
- Initial Rust structure setup

### 0.0.1 (Previous)
- Project initialization

---

For the original Python version history, see [CHANGES.md](CHANGES.md).