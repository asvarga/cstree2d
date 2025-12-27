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

/// Parse a comment-and-space-indented block of text.
fn parse<'a>(builder: &mut Builder<TestSyntax>, s: &'a str) {
    builder.start_node(TestSyntax::Root);

    let mut indents = vec![];
    for mut line in s.lines() {
        // chomp all matching indents out of the line
        let mut kept = 0;
        for indent in &indents {
            if let Some(rest) = line.strip_prefix(indent) {
                line = rest;
                kept += 1;
            } else {
                break;
            }
        }

        // dedent all unkept indents
        builder.dedents(indents.len() - kept);
        indents.truncate(kept);

        // indent if needed
        let new_indent_len = line.chars().take_while(|c| *c == ' ' || *c == '#').count();
        if new_indent_len > 0 {
            let (indent, rest) = line.split_at(new_indent_len);
            builder.indent(indent);
            indents.push(indent);
            line = rest;
        }

        // add the text token + newline
        if line.len() > 0 {
            builder.token(TestSyntax::Text, line);
        }
        builder.newline();
    }

    // dedent all the way
    for _ in 0..indents.len() {
        builder.dedent();
    }

    builder.finish_node();
}

/**************************************************************/

fn main() {
    let s = indoc! {"
        # A simple example
        def hello_world():
            print('Hello')
            print('World')
        hello_world()
    "};

    let mut builder = Builder::<TestSyntax>::new();
    parse(&mut builder, s);

    let red = builder.red();
    println!("{}", red.debug(true));
    println!("{red}");
    assert_eq!(red.to_string(), s);
}
