use cstree2d::{cstree::Syntax, green::Builder};
use indoc::indoc;

/**************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Syntax)]
#[repr(u32)]
pub(crate) enum TestSyntax {
    Root,
    Text,
}

/**************************************************************/

/// Parse a space-indented block of text.
fn parse<'a>(builder: &mut Builder<TestSyntax>, s: &'a str) {
    builder.start_node(TestSyntax::Root);

    let mut indent = 0;
    for line in s.lines() {
        let leading_spaces = line.chars().take_while(|c| *c == ' ').count();

        for _ in 0..(leading_spaces.saturating_sub(indent)) {
            builder.indent(" ");
        }
        for _ in 0..(indent.saturating_sub(leading_spaces)) {
            builder.dedent();
        }
        indent = leading_spaces;
        builder.token(TestSyntax::Text, &line[leading_spaces..]);
        builder.newline();
    }

    builder.finish_node();
}

/**************************************************************/

fn main() {
    let s = indoc! {"
        def hello_world():
            print('Hello')
            print('World')
        hello_world()
    "};

    let mut builder: Builder<TestSyntax> = Builder::new();
    parse(&mut builder, s);

    let red = builder.red();
    println!("{}", red.debug(true));
    println!("{red}");
}
