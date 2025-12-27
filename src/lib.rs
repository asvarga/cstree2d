//! A wrapper around `cstree` with support for indentation tracking.
//!
//! This crate provides special token types for managing indentation-aware
//! syntax trees, particularly useful for indentation-sensitive languages.

use cstree::{
    Syntax,
    build::{GreenNodeBuilder, NodeCache},
    green::GreenNode,
    interning::Interner,
};
use std::fmt::{self, Display};

/// Special token kinds for indentation-aware syntax trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Syntax2D<S> {
    /// Adds a string slice as indentation to the stack (e.g., "    " or "# ")
    Indent,
    /// Removes a string slice from the indentation stack
    Dedent,
    /// Represents a newline character
    Newline,
    /// A normal token (should not contain newlines)
    Token(S),
}

impl<S: Syntax> Syntax for Syntax2D<S> {
    fn from_raw(raw: cstree::RawSyntaxKind) -> Self {
        match raw.0 {
            x if x == u32::MAX - 2 => Syntax2D::Indent,
            x if x == u32::MAX - 1 => Syntax2D::Dedent,
            x if x == u32::MAX => Syntax2D::Newline,
            _ => Syntax2D::Token(S::from_raw(raw)),
        }
    }

    fn into_raw(self) -> cstree::RawSyntaxKind {
        match self {
            Syntax2D::Indent => cstree::RawSyntaxKind(u32::MAX - 2),
            Syntax2D::Dedent => cstree::RawSyntaxKind(u32::MAX - 1),
            Syntax2D::Newline => cstree::RawSyntaxKind(u32::MAX),
            Syntax2D::Token(s) => s.into_raw(),
        }
    }

    fn static_text(self) -> Option<&'static str> {
        match self {
            Syntax2D::Token(s) => s.static_text(),
            _ => None,
        }
    }
}

impl<S: Syntax> From<S> for Syntax2D<S> {
    fn from(s: S) -> Self {
        Syntax2D::Token(s)
    }
}

impl<S: Syntax + Display> fmt::Display for Syntax2D<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Syntax2D::Indent => write!(f, "Indent"),
            Syntax2D::Dedent => write!(f, "Dedent"),
            Syntax2D::Newline => write!(f, "Newline"),
            Syntax2D::Token(text) => write!(f, "Text({text})"),
        }
    }
}

/// Builder for creating indentation-aware syntax trees.
///
/// This wraps `GreenNodeBuilder` and provides convenience methods for
/// managing indentation tokens.
pub struct Builder<'cache, 'interner, S: Syntax, I: Interner = cstree::interning::TokenInterner> {
    inner: GreenNodeBuilder<'cache, 'interner, Syntax2D<S>, I>,
}

impl<S: Syntax> Builder<'static, 'static, S> {
    /// Creates a new builder with default settings.
    pub fn new() -> Self {
        Self {
            inner: GreenNodeBuilder::new(),
        }
    }
}

impl<S: Syntax> Default for Builder<'static, 'static, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'cache, 'interner, S: Syntax, I: Interner> Builder<'cache, 'interner, S, I> {
    /// Creates a new builder with a custom cache.
    pub fn with_cache(cache: &'cache mut NodeCache<'interner, I>) -> Self {
        Self {
            inner: GreenNodeBuilder::with_cache(cache),
        }
    }

    /// Creates a new builder with a custom interner.
    pub fn with_interner(interner: &'interner mut I) -> Self {
        Self {
            inner: GreenNodeBuilder::with_interner(interner),
        }
    }

    /// Starts a new node with the given inner syntax kind.
    ///
    /// This is a convenience method equivalent to `start_node(Syntax2D::Token(kind))`.
    pub fn start_node(&mut self, kind: S) {
        self.inner.start_node(Syntax2D::Token(kind));
    }

    /// Finishes the current node.
    pub fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    /// Adds a token to the current node with the given inner syntax kind.
    ///
    /// This is a convenience method equivalent to `token(Syntax2D::Token(kind), text)`.
    pub fn token(&mut self, kind: S, text: &str) {
        self.inner.token(Syntax2D::Token(kind), text);
    }

    /// Adds an indent token with the given indentation string.
    pub fn indent(&mut self, indent_str: &str) {
        self.inner.token(Syntax2D::Indent, indent_str);
    }

    /// Adds a dedent token.
    ///
    /// Note: The dedent token stores an empty string since only the indent token
    /// needs to track the indentation text for proper text extraction.
    pub fn dedent(&mut self) {
        self.inner.token(Syntax2D::Dedent, "");
    }

    /// Adds a newline token.
    pub fn newline(&mut self) {
        self.inner.token(Syntax2D::Newline, "\n");
    }

    /// Finishes building and returns the root green node.
    pub fn finish(self) -> (GreenNode, Option<NodeCache<'interner, I>>) {
        self.inner.finish()
    }
}

/// Extracts formatted text from a `GreenNode` with indentation applied.
///
/// This walks the tree and:
/// - Outputs text from `Text` tokens directly
/// - Tracks indentation from `Indent`/`Dedent` tokens
/// - Applies current indentation after each `Newline` token
///
/// # Parameters
/// * `node` - The root node to extract text from
/// * `resolver` - A resolver for looking up interned tokens (use the cache from `Builder::finish`)
pub fn extract_text<S: Syntax, I: Interner>(node: &GreenNode, resolver: &I) -> String {
    let mut output = String::new();
    let mut indentation_stack: Vec<String> = Vec::new();
    let mut pending_indentation = false;

    fn walk<S: Syntax, R: cstree::interning::Resolver>(
        node: &GreenNode,
        output: &mut String,
        indentation_stack: &mut Vec<String>,
        pending_indentation: &mut bool,
        resolver: &R,
    ) {
        use cstree::util::NodeOrToken;

        for child in node.children() {
            match child {
                NodeOrToken::Token(token) => {
                    let kind = Syntax2D::<S>::from_raw(token.kind());
                    let text = token.text(resolver).unwrap_or("");

                    match kind {
                        Syntax2D::Indent => {
                            indentation_stack.push(text.to_string());
                            *pending_indentation = true;
                        }
                        Syntax2D::Dedent => {
                            indentation_stack.pop();
                        }
                        Syntax2D::Newline => {
                            output.push('\n');
                            *pending_indentation = !indentation_stack.is_empty();
                        }
                        Syntax2D::Token(_) => {
                            if *pending_indentation {
                                output.push_str(&indentation_stack.concat());
                                *pending_indentation = false;
                            }
                            output.push_str(text);
                        }
                    }
                }
                NodeOrToken::Node(child_node) => {
                    walk::<S, R>(
                        child_node,
                        output,
                        indentation_stack,
                        pending_indentation,
                        resolver,
                    );
                }
            }
        }
    }

    walk::<S, I>(
        node,
        &mut output,
        &mut indentation_stack,
        &mut pending_indentation,
        resolver,
    );

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum TestSyntax {
        Root,
        Text,
    }

    impl Syntax for TestSyntax {
        fn from_raw(raw: cstree::RawSyntaxKind) -> Self {
            match raw.0 {
                0 => TestSyntax::Root,
                1 => TestSyntax::Text,
                _ => panic!("Invalid raw syntax kind: {}", raw.0),
            }
        }

        fn into_raw(self) -> cstree::RawSyntaxKind {
            cstree::RawSyntaxKind(self as u32)
        }

        fn static_text(self) -> Option<&'static str> {
            None
        }
    }

    #[test]
    fn test_basic_indentation() {
        let mut builder: Builder<TestSyntax> = Builder::new();
        builder.start_node(TestSyntax::Root);

        builder.token(TestSyntax::Text, "line1");
        builder.newline();

        builder.indent("    ");
        builder.token(TestSyntax::Text, "indented");
        builder.newline();

        builder.token(TestSyntax::Text, "still_indented");
        builder.dedent();

        builder.finish_node();
        let (root, _cache) = builder.finish();

        assert!(root.children().count() > 0);
    }

    #[test]
    fn test_dump_text_simple() {
        let mut builder: Builder<TestSyntax> = Builder::new();
        builder.start_node(TestSyntax::Root);

        builder.token(TestSyntax::Text, "hello");
        builder.token(TestSyntax::Text, " ");
        builder.token(TestSyntax::Text, "world");

        builder.finish_node();
        let (root, cache) = builder.finish();

        let resolver = cache.expect("No cache");
        let text = extract_text::<TestSyntax, _>(&root, resolver.interner());
        assert_eq!(text, "hello world");
    }

    #[test]
    fn test_dump_text_with_newlines() {
        let mut builder: Builder<TestSyntax> = Builder::new();
        builder.start_node(TestSyntax::Root);

        builder.token(TestSyntax::Text, "line1");
        builder.newline();
        builder.token(TestSyntax::Text, "line2");

        builder.finish_node();
        let (root, cache) = builder.finish();

        let resolver = cache.expect("No cache");
        let text = extract_text::<TestSyntax, _>(&root, resolver.interner());
        assert_eq!(
            text,
            indoc! {"
            line1
            line2"
            }
        );
    }

    #[test]
    fn test_dump_text_with_indentation() {
        let mut builder: Builder<TestSyntax> = Builder::new();
        builder.start_node(TestSyntax::Root);

        builder.token(TestSyntax::Text, "line1");
        builder.newline();

        builder.indent("    ");
        builder.token(TestSyntax::Text, "indented");
        builder.newline();
        builder.token(TestSyntax::Text, "still_indented");
        builder.dedent();

        builder.finish_node();
        let (root, cache) = builder.finish();

        let resolver = cache.expect("No cache");
        let text = extract_text::<TestSyntax, _>(&root, resolver.interner());
        assert_eq!(
            text,
            indoc! {"
            line1
                indented
                still_indented"
            }
        );
    }

    #[test]
    fn test_dump_text_with_nested_indentation() {
        let mut builder: Builder<TestSyntax> = Builder::new();
        builder.start_node(TestSyntax::Root);

        builder.token(TestSyntax::Text, "start");
        builder.newline();

        builder.indent("  ");
        builder.token(TestSyntax::Text, "level1");
        builder.newline();

        builder.indent("  ");
        builder.token(TestSyntax::Text, "level2");
        builder.newline();

        builder.token(TestSyntax::Text, "still_level2");
        builder.dedent();
        builder.newline();

        builder.token(TestSyntax::Text, "back_to_level1");
        builder.dedent();
        builder.newline();

        builder.token(TestSyntax::Text, "end");

        builder.finish_node();
        let (root, cache) = builder.finish();

        let resolver = cache.expect("No cache");
        let text = extract_text::<TestSyntax, _>(&root, resolver.interner());
        assert_eq!(
            text,
            indoc! {"
            start
              level1
                level2
                still_level2
              back_to_level1
            end"
            }
        );
    }

    #[test]
    fn test_dump_text_mixed_indentation_styles() {
        let mut builder: Builder<TestSyntax> = Builder::new();
        builder.start_node(TestSyntax::Root);

        builder.token(TestSyntax::Text, "start");
        builder.newline();

        builder.indent("    ");
        builder.indent("# ");
        builder.token(TestSyntax::Text, "comment");
        builder.dedent();
        builder.dedent();

        builder.finish_node();
        let (root, cache) = builder.finish();

        let resolver = cache.expect("No cache");
        let text = extract_text::<TestSyntax, _>(&root, resolver.interner());
        assert_eq!(
            text,
            indoc! {"
            start
                # comment"
            }
        );
    }

    #[test]
    fn test_syntax_size() {
        // niche optimization should ensure both have the same size
        assert_eq!(
            std::mem::size_of::<Syntax2D<TestSyntax>>(),
            std::mem::size_of::<TestSyntax>()
        );
    }
}
