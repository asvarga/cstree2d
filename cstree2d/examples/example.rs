use cstree2d::{cstree::Syntax, syntax::Syntax2D};

/**************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Syntax)]
#[repr(u32)]
pub(crate) enum TestSyntax {
    Root,
    Text,
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
