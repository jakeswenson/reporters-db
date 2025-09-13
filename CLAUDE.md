# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Rust port of the Free Law Project's database of court reporters. It provides data and utilities for identifying and parsing legal citations in text.

## Development Commands

### Build and Test
- `cargo build` - Build the library
- `cargo test` - Run all Rust tests
- `cargo test <test_name>` - Run a specific test
- `cargo clippy` - Run linter for Rust code
- `cargo fmt` - Format Rust code
- `python tests.py` - Run Python validation tests for the JSON data

### Pre-commit Hooks
The project uses pre-commit hooks configured with cargo-husky that automatically run:
- `cargo test`
- `cargo clippy`
- `cargo fmt`

## Architecture

### Core Structure
The library is organized around different types of legal citations with corresponding data and utilities:

- **reporters** - Database of court reporters (e.g., "U.S.", "F.3d") with metadata about editions, dates, and variations
- **journals** - Legal journal abbreviations and their full names
- **laws** - Statutory citation formats and patterns
- **regexes** - Template-based regex patterns for matching citations, with placeholders resolved from `schemas/regexes.json`
- **state_abbreviations** - Standard abbreviations for U.S. states
- **case_name_part_abbreviations** - Common abbreviations found in case names

### Data Sources
The JSON data in `schemas/` directory contains the actual reporter database information. The Rust code in `src/` provides typed access to this data with lazy static loading and regex compilation.

### Key Design Patterns
- Uses `lazy_static` for one-time initialization of large data structures
- Regex templates support placeholder substitution from `regexes.json` for reusable patterns
- All modules expose public utility functions through `src/utils.rs`
- Error handling uses `thiserror` for custom error types