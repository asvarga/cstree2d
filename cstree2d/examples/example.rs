use cstree2d::{
    cstree::{RawSyntaxKind, Syntax},
    syntax::Syntax2D,
};

/**************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TestSyntax {
    Root,
    Text,
}

impl Syntax for TestSyntax {
    fn from_raw(raw: RawSyntaxKind) -> Self {
        match raw.0 {
            0 => TestSyntax::Root,
            1 => TestSyntax::Text,
            _ => panic!("Invalid raw syntax kind: {}", raw.0),
        }
    }

    fn into_raw(self) -> RawSyntaxKind {
        RawSyntaxKind(self as u32)
    }

    fn static_text(self) -> Option<&'static str> {
        None
    }
}

type TestSyntax2D = Syntax2D<TestSyntax>;

/**************************************************************/

fn main() {
    let tokens = [
        TestSyntax2D::Indent,
        TestSyntax2D::Dedent,
        TestSyntax2D::Newline,
        TestSyntax::Root.into(),
        TestSyntax::Text.into(),
    ];

    for token in tokens {
        println!("{token:?}");
    }
}
