//! src/lib.rs
//!
//! The root of the recommendation crate: re-exports data loading and recommendation logic
//! so that external binaries and tests can import a single entry point.

/// Provides data-loading utilities and core record types.
pub mod data;

/// Implements the collaborative-filtering recommendation algorithms.
pub mod recommend;