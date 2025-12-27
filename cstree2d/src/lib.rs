//! A wrapper around `cstree` with support for indentation tracking.
//!
//! This crate provides special token types for managing indentation-aware
//! syntax trees, particularly useful for indentation-sensitive languages.

pub use cstree;

pub mod green;
pub mod red;
pub mod syntax;
