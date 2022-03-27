pub mod reporters;
pub mod state_abbreviations;

pub mod case_name_part_abbreviations;

pub mod journals;
pub mod laws;

mod regexes;

mod utils;

pub use regexes::{regexes, RegexTemplate};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Failed to resolve regex because there was too much recursion in the templates")]
    TooMuchRecursion,
}
