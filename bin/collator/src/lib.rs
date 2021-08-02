//! Astar collator library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

/// Genesis token distributions.
mod balances;
/// Chain specifications.
pub mod chain_spec;

#[macro_use]
mod service;
mod cli;
mod command;

pub use cli::*;
pub use command::*;