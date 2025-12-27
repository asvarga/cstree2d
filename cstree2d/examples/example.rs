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

    let mut indents = vec![0];
    for line in s.lines() {
        let new_indent = line.chars().take_while(|c| *c == ' ').count();
        // do as many dedents as needed
        while indents.last().unwrap() > &new_indent {
            builder.dedent();
            indents.pop();
        }
        // maybe do an indent
        if indents.last().unwrap() < &new_indent {
            builder.indent(&line[indents.last().unwrap().to_owned()..new_indent]);
            indents.push(new_indent);
        }
        builder.token(TestSyntax::Text, &line[new_indent..]);
        builder.newline();
    }
    // dedent all the way
    for _ in 1..indents.len() {
        builder.dedent();
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
