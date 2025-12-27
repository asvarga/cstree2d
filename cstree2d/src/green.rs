use crate::{red::ResolvedNode2D, syntax::Syntax2D};
use cstree::{
    Syntax,
    build::{GreenNodeBuilder, NodeCache},
    green::GreenNode,
    interning::{Interner, Resolver, TokenInterner},
    syntax::ResolvedNode,
    util::NodeOrToken,
};
use std::fmt::Formatter;

/**************************************************************/

/// Builder for creating indentation-aware syntax trees.
///
/// This wraps `GreenNodeBuilder` and provides convenience methods for
/// managing indentation tokens.
pub struct Builder<'cache, 'interner, S: Syntax, I: Interner = TokenInterner> {
    inner: GreenNodeBuilder<'cache, 'interner, Syntax2D<S>, I>,
}

impl<S: Syntax> Builder<'static, 'static, S> {
    /// Creates a new builder with default settings.
    pub fn new() -> Self {
        Self {
            inner: GreenNodeBuilder::new(),
        }
    }

    pub fn inner(&self) -> &GreenNodeBuilder<'static, 'static, Syntax2D<S>, TokenInterner> {
        &self.inner
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
        self.inner.static_token(Syntax2D::Dedent);
    }

    /// Adds multiple dedent tokens.
    pub fn dedents(&mut self, i: usize) {
        for _ in 0..i {
            self.dedent();
        }
    }

    /// Adds a newline token.
    pub fn newline(&mut self) {
        self.inner.static_token(Syntax2D::Newline);
    }

    /// Finishes building and returns the root green node.
    pub fn finish(self) -> (GreenNode, Option<NodeCache<'interner, I>>) {
        self.inner.finish()
    }
}

impl<S: Syntax, I: Interner + 'static> Builder<'static, 'static, S, I> {
    /// Consumes the builder and returns a resolved red tree node.
    ///
    /// This method is only available when the builder owns its interner (i.e., when both
    /// lifetimes are `'static`), which is the case when using `Builder::new()`.
    pub fn red(self) -> ResolvedNode2D<S> {
        let (green, cache) = self.finish();
        let interner = cache.unwrap().into_interner().unwrap();
        let root: ResolvedNode<Syntax2D<S>> = ResolvedNode::new_root_with_resolver(green, interner);
        ResolvedNode2D::new(root)
    }
}

/// Extracts formatted text from a `GreenNode` with indentation applied, writing to a formatter.
///
/// This walks the tree and:
/// - Outputs text from `Text` tokens directly
/// - Tracks indentation from `Indent`/`Dedent` tokens
/// - Applies current indentation after each `Newline` token
///
/// # Parameters
/// * `node` - The root node to extract text from
/// * `resolver` - A resolver for looking up interned tokens (use the cache from `Builder::finish`)
/// * `f` - The formatter to write the output to
pub fn extract_text<S: Syntax, R: Resolver + ?Sized>(
    node: &GreenNode,
    resolver: &R,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    fn walk<'a, S: Syntax, R: Resolver + ?Sized>(
        node: &'a GreenNode,
        f: &mut Formatter<'_>,
        indentation_stack: &mut Vec<&'a str>,
        pending_indentation: &mut bool,
        resolver: &'a R,
    ) -> std::fmt::Result {
        for child in node.children() {
            match child {
                NodeOrToken::Token(token) => {
                    let kind = Syntax2D::<S>::from_raw(token.kind());
                    match kind {
                        Syntax2D::Indent => {
                            indentation_stack.push(token.text(resolver).unwrap());
                            *pending_indentation = true;
                        }
                        Syntax2D::Dedent => {
                            indentation_stack.pop();
                        }
                        Syntax2D::Newline => {
                            if *pending_indentation {
                                for indent in indentation_stack.iter() {
                                    f.write_str(indent)?;
                                }
                            }
                            f.write_str("\n")?;
                            *pending_indentation = !indentation_stack.is_empty();
                        }
                        Syntax2D::Token(_) => {
                            if *pending_indentation {
                                for indent in indentation_stack.iter() {
                                    f.write_str(indent)?;
                                }
                                *pending_indentation = false;
                            }
                            f.write_str(token.text(resolver).unwrap())?;
                        }
                    }
                }
                NodeOrToken::Node(child_node) => {
                    walk::<S, R>(
                        child_node,
                        f,
                        indentation_stack,
                        pending_indentation,
                        resolver,
                    )?;
                }
            }
        }
        Ok(())
    }

    let mut indentation_stack = vec![];
    let mut pending_indentation = false;
    walk::<S, R>(
        node,
        f,
        &mut indentation_stack,
        &mut pending_indentation,
        resolver,
    )
}
