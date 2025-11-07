//! # Intelligence Path-Finding Library
//!
//! A comprehensive library for implementing and visualizing path-finding algorithms.
//! This library provides multiple algorithms with interactive visualization capabilities.

pub mod algorithms;
pub mod data_structures;
pub mod heuristics;
pub mod visualization;

// Re-export core types for convenience
pub use algorithms::*;
pub use data_structures::*;
pub use heuristics::*;
pub use visualization::*;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");