use cstree2d::{cstree::Syntax, green::Builder};

/**************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Syntax)]
#[repr(u32)]
pub(crate) enum TestSyntax {
    Root,
    Text,
}

/**************************************************************/

fn main() {
    let mut builder: Builder<TestSyntax> = Builder::new();
    builder.start_node(TestSyntax::Root);

    builder.token(TestSyntax::Text, "def hello_world():");
    builder.newline();
    builder.indent("    ");

    builder.token(TestSyntax::Text, "print('Hello')");
    builder.newline();

    builder.token(TestSyntax::Text, "print('World')");
    builder.newline();
    builder.dedent();

    builder.token(TestSyntax::Text, "hello_world()");
    builder.newline();

    builder.finish_node();

    let red = builder.red();
    println!("{red}");
}
