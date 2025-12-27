//! A wrapper around `cstree` with support for indentation tracking.
//!
//! This crate provides special token types for managing indentation-aware
//! syntax trees, particularly useful for indentation-sensitive languages.

pub use cstree;

pub mod green;
pub mod red;
pub mod syntax;

/**************************************************************/

use crate::green::extract_text;
use cstree::{
    Syntax,
    green::GreenNode,
    interning::{Interner, Resolver},
};
use std::fmt::{Display, Formatter};

/**************************************************************/

/// Extracts formatted text from a `GreenNode` with indentation applied as a String.
///
/// This is a convenience wrapper around `extract_text` that returns a String.
///
/// # Parameters
/// * `node` - The root node to extract text from
/// * `resolver` - A resolver for looking up interned tokens (use the cache from `Builder::finish`)
pub fn extract_text_to_string<S: Syntax, I: Interner>(node: &GreenNode, resolver: &I) -> String {
    use std::fmt::Write;
    let mut output = String::new();
    write!(
        &mut output,
        "{}",
        TextDisplay {
            node,
            resolver,
            _phantom: std::marker::PhantomData::<S>
        }
    )
    .expect("Writing to String should not fail");
    output
}

/// Helper struct to implement Display for formatted text extraction
struct TextDisplay<'a, S, I: ?Sized> {
    node: &'a GreenNode,
    resolver: &'a I,
    _phantom: std::marker::PhantomData<S>,
}

impl<S: Syntax, I: Resolver + ?Sized> Display for TextDisplay<'_, S, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        extract_text::<S, I>(self.node, self.resolver, f)
    }
}
