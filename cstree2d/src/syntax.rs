use cstree::{RawSyntaxKind, Syntax};

/**************************************************************/

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
    fn from_raw(raw: RawSyntaxKind) -> Self {
        match raw.0 {
            x if x == u32::MAX - 2 => Syntax2D::Indent,
            x if x == u32::MAX - 1 => Syntax2D::Dedent,
            x if x == u32::MAX => Syntax2D::Newline,
            _ => Syntax2D::Token(S::from_raw(raw)),
        }
    }

    fn into_raw(self) -> RawSyntaxKind {
        match self {
            Syntax2D::Indent => RawSyntaxKind(u32::MAX - 2),
            Syntax2D::Dedent => RawSyntaxKind(u32::MAX - 1),
            Syntax2D::Newline => RawSyntaxKind(u32::MAX),
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
