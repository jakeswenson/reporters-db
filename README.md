# Reporters Database (Rust Port)

[![Crates.io](https://img.shields.io/crates/v/reporters-db)](https://crates.io/crates/reporters-db)
[![Documentation](https://docs.rs/reporters-db/badge.svg)](https://docs.rs/reporters-db)
[![License](https://img.shields.io/crates/l/reporters-db)](https://github.com/jakeswenson/reporters-db)

A high-performance Rust port of the [Free Law Project's](https://free.law) comprehensive database of court reporters. This library provides zero-overhead access to legal citation data through compile-time Perfect Hash Functions (PHF).

## Features

- **Zero Runtime Overhead**: All data embedded at compile time using PHF
- **Comprehensive Database**: 4,000+ court reporters, journals, and legal publications
- **100% API Compatibility**: Drop-in replacement for the Python version
- **Type Safety**: Full Rust type system with comprehensive documentation
- **Performance**: O(1) lookups with static memory allocation

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
reporters-db = "0.0.5"
```

### Basic Usage

```rust
use reporters_db::{get_reporters, get_variations_only, CiteType};

// Get the main reporters database
let reporters = get_reporters();
if let Some(reporter_list) = reporters.get("F.3d") {
    for reporter in reporter_list.iter() {
        println!("Reporter: {}", reporter.name);
        match reporter.cite_type {
            CiteType::Federal => println!("This is a federal court reporter"),
            CiteType::State => println!("This is a state court reporter"),
            _ => println!("Other type: {:?}", reporter.cite_type),
        }
    }
}

// Get variation mappings
let variations = get_variations_only();
if let Some(canonical_forms) = variations.get("Atlantic Reporter") {
    println!("Canonical forms: {:?}", canonical_forms);
}
```

### Advanced Usage

```rust
use reporters_db::{get_names_to_editions, get_regex_variables, get_journals};

// Get reporter name to editions mapping
let names_to_editions = get_names_to_editions();
if let Some(editions) = names_to_editions.get("Atlantic Reporter") {
    println!("Atlantic Reporter editions: {:?}", editions);
    // Output: ["A.", "A.2d", "A.3d"]
}

// Get regex patterns for citation parsing
let regex_vars = get_regex_variables();
if let Some(template) = regex_vars.get("full_cite") {
    println!("Full citation pattern: {}", template.value());
}

// Access journals database
let journals = get_journals();
for (key, journal_list) in journals.entries().take(5) {
    println!("Journal '{}' has {} entries", key, journal_list.len());
}
```

## API Reference

### Main Functions

| Function | Description |
|----------|-------------|
| [`get_reporters()`] | Main court reporters database |
| [`get_variations_only()`] | Variation to canonical name mappings |
| [`get_editions()`] | Edition to parent reporter mappings |
| [`get_names_to_editions()`] | Reporter names to edition lists |
| [`get_journals()`] | Academic journals and law reviews |
| [`get_laws()`] | Statutory law and code publications |
| [`get_regex_variables()`] | Citation parsing regex templates |

### Key Types

- **[`Reporter`]** - Court reporter with editions and metadata
- **[`CiteType`]** - Citation type classification (Federal, State, etc.)
- **[`Edition`]** - Specific edition/series of a reporter
- **[`Journal`]** - Academic legal publication
- **[`Law`]** - Statutory law or code reference

## Performance

This Rust port provides significant performance improvements over the Python version:

- **Compile-time data embedding**: No JSON parsing at runtime
- **Perfect hash functions**: O(1) lookups with zero hash collisions
- **Static memory allocation**: No dynamic memory allocation during queries
- **Zero-copy string access**: All strings are `&'static str` references

## Background

Court reporters are books containing judicial opinions, dating back centuries. Famous early reporters include those by [William Cranch](https://en.wikipedia.org/wiki/William_Cranch) and [Alexander Dallas](https://en.wikipedia.org/wiki/Alexander_J._Dallas_%28statesman%29). This database catalogs thousands of these publications, from West's reporters to regional series like "Dakota Reports."

The original Python database is maintained by the [Free Law Project](https://free.law). This Rust port provides the same comprehensive data with significant performance improvements for systems requiring high-throughput legal citation processing.

## Data Sources

- **Court Reporters**: Federal, state, and regional court decision publications
- **Journals**: Law reviews, academic journals, bar publications
- **Laws**: Statutory codes, regulations, and legal statutes
- **Variations**: Alternative names and abbreviations for publications
- **Citation Patterns**: Regex templates for parsing legal citations

## Contributing

This is a port of the [original reporters-db](https://github.com/freelawproject/reporters-db) Python library. For data corrections or additions, please contribute to the upstream Python project.

For Rust-specific issues, performance improvements, or API enhancements, please open issues in this repository.

## License

This project maintains the same license as the original Free Law Project database.

## Links

- [Original Python Library](https://github.com/freelawproject/reporters-db)
- [Free Law Project](https://free.law)
- [Documentation](https://docs.rs/reporters-db)
- [Crates.io](https://crates.io/crates/reporters-db)