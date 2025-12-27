# cstree2d

A wrapper around `cstree` with support for indentation tracking.

## Overview

This crate provides a `Builder` type that wraps `cstree::GreenNodeBuilder` and maintains an indentation stack to track the current indentation level and style. It uses a simple `Syntax2D` enum as the syntax type, making it easy to build indentation-aware syntax trees. This is particularly useful for indentation-sensitive languages like Python, YAML, or any custom DSL.

## Features

- **Indent tracking**: Push indentation strings (e.g., `"    "`, `"  "`, `"# "`) onto a stack
- **Dedent tracking**: Pop indentation from the stack
- **Newline handling**: Automatically include current indentation after newlines
- **Text output**: Get properly formatted text with indentation applied automatically
- **Simple API**: Uses a concrete `Syntax2D` enum - no need for trait implementations

## Usage

```rust
use cstree2d::{Builder, Syntax2D};

fn main() {
    let mut builder = Builder::new();
    
    builder.start_node(Syntax2D::Text);
    
    // Add some text
    builder.token(Syntax2D::Text, "line1");
    builder.newline(false);
    
    // Add indentation
    builder.indent("    ");
    builder.token(Syntax2D::Text, "indented_line");
    
    // Newline (indentation automatically applied)
    builder.newline(false);
    builder.token(Syntax2D::Text, "still_indented");
    
    // Remove indentation
    builder.dedent();
    
    builder.finish_node();
    
    // Get the formatted text output
    let text = builder.text_output();
    assert_eq!(text, "line1\n    indented_line\n    still_indented");
    
    // Or finish and get both the green node and text
    let (root, _cache, text) = builder.finish();
}
```

## API

### Syntax2D Enum

The `Syntax2D` enum represents the different token types:

```rust
pub enum Syntax2D {
    Indent,  // Adds indentation to the stack
    Dedent,  // Removes indentation from the stack
    Newline, // Represents a newline character
    Text,    // A normal token (should not contain newlines)
}
```

### Builder Methods

- `new()` - Create a new builder with default settings
- `with_cache(cache)` - Create a builder with a custom node cache
- `with_interner(interner)` - Create a builder with a custom interner
- `start_node(kind)` - Start a new node with the given syntax kind
- `finish_node()` - Finish the current node
- `token(kind, text)` - Add a token to the current node
- `indent(indent_str)` - Push indentation onto the stack
- `dedent()` - Pop indentation from the stack
- `newline(with_indent)` - Add a newline (indentation automatically applied to text output)
- `current_indentation()` - Get the current indentation as a concatenated string
- `indentation_level()` - Get the number of indent tokens on the stack
- `indentation_stack()` - Get a reference to the indentation stack
- `clear_indentation()` - Clear the indentation stack
- `text_output()` - Get the accumulated text with proper indentation
- `finish()` - Finish building and return the root green node, optional cache, and formatted text

## How It Works

The builder tracks indentation in two ways:

1. **In the CST**: Indent, dedent, and newline tokens are stored in the tree with their original text
2. **In text output**: The builder accumulates properly formatted text where:
   - Indent tokens push to the stack and mark pending indentation
   - Dedent tokens pop from the stack
   - Newline tokens add `\n` and mark pending indentation
   - Text tokens automatically get pending indentation added before them

This allows you to:
- Build a complete CST with all indentation information
- Get nicely formatted text output automatically
- Query the tree structure while preserving indentation semantics

## Examples

### Basic indentation

```rust
let mut builder = Builder::new();
builder.start_node(Syntax2D::Text);

builder.token(Syntax2D::Text, "def foo():");
builder.newline(false);
builder.indent("    ");
builder.token(Syntax2D::Text, "return 42");
builder.dedent();

builder.finish_node();
let text = builder.text_output();
assert_eq!(text, "def foo():\n    return 42");
```

### Nested indentation

```rust
let mut builder = Builder::new();
builder.start_node(Syntax2D::Text);

builder.token(Syntax2D::Text, "if True:");
builder.newline(false);

builder.indent("  ");
builder.token(Syntax2D::Text, "if False:");
builder.newline(false);

builder.indent("  ");
builder.token(Syntax2D::Text, "pass");
builder.dedent();
builder.dedent();

builder.finish_node();
let text = builder.text_output();
assert_eq!(text, "if True:\n  if False:\n    pass");
```

### Mixed indentation styles

```rust
let mut builder = Builder::new();
builder.start_node(Syntax2D::Text);

builder.token(Syntax2D::Text, "code");
builder.newline(false);

builder.indent("    ");
builder.indent("# ");
builder.token(Syntax2D::Text, "comment inside indent");
builder.dedent();
builder.dedent();

builder.finish_node();
let text = builder.text_output();
assert_eq!(text, "code\n    # comment inside indent");
```

## License

Same as the parent project.
