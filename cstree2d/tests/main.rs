use cstree2d::{Builder, cstree::Syntax, extract_text, syntax::Syntax2D};
use indoc::indoc;

/**************************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Syntax)]
#[repr(u32)]
pub(crate) enum TestSyntax {
    Root,
    Text,
}
type TestSyntax2D = Syntax2D<TestSyntax>;

/**************************************************************/

#[test]
fn test_syntax_size() {
    // niche optimization should ensure both have the same size
    assert_eq!(
        std::mem::size_of::<TestSyntax2D>(),
        std::mem::size_of::<TestSyntax>()
    );
}

/**************************************************************/

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
