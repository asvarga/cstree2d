use crate::{green::extract_text, syntax::Syntax2D};
use cstree::{
    Syntax,
    green::GreenNode,
    syntax::{ResolvedNode, SyntaxNode},
};
use std::fmt::{Display, Formatter};

/**************************************************************/

/// A "red" node wrapper around `ResolvedNode<Syntax2D<S>>`.
pub struct ResolvedNode2D<S: Syntax> {
    inner: ResolvedNode<Syntax2D<S>>,
}

impl<S: Syntax> ResolvedNode2D<S> {
    pub fn new(inner: ResolvedNode<Syntax2D<S>>) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &ResolvedNode<Syntax2D<S>> {
        &self.inner
    }

    pub fn debug(&self, recursive: bool) -> String {
        let resolver = self.inner.resolver();
        self.inner.debug(&**resolver, recursive)
    }

    /// Returns the unterlying green tree node of this node.
    pub fn green(&self) -> &GreenNode {
        self.inner.green()
    }
}

impl<S: Syntax> Display for ResolvedNode2D<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let resolver = self.inner.resolver();
        extract_text::<S, _>(self.green(), &**resolver, f)
    }
}

/**************************************************************/

/// A "red" node wrapper around `SyntaxNode<Syntax2D<S>>`.
pub struct SyntaxNode2D<S: Syntax> {
    inner: SyntaxNode<Syntax2D<S>>,
}

impl<S: Syntax> SyntaxNode2D<S> {
    pub fn new(inner: SyntaxNode<Syntax2D<S>>) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &SyntaxNode<Syntax2D<S>> {
        &self.inner
    }

    pub fn debug(&self, recursive: bool) -> Option<String> {
        let resolver = self.inner.resolver()?;
        Some(self.inner.debug(&**resolver, recursive))
    }

    /// Returns the unterlying green tree node of this node.
    pub fn green(&self) -> &GreenNode {
        self.inner.green()
    }
}

impl<S: Syntax> Display for SyntaxNode2D<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let resolver = self.inner.resolver().ok_or(std::fmt::Error)?;
        extract_text::<S, _>(self.green(), &**resolver, f)
    }
}
