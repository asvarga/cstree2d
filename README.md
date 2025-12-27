# cstree2d

A simple wrapper around `cstree` with support for indentation tracking.

## Benefits

Adding indent/dedent tokens to a language makes it two-dimensional: rectangular code regions can be shifted up/down/left/right in a text document without losing their local meaning. This translational invariance could be useful for:

- Programmatic manipulation of CSTs that maintains formatting
- Tokenization of training data for LLMs
- Duplicate code detection with Rabin-Karp
- The processing of mutli-language files such as Markdown or React

For example, these blocks of code contain the same python function, but most tools would fail to recognize it:

```py
def hello_world():
    print('Hello')
    print('World')
```

```rs
> ```rs
> /// This function is equivalent to the python function:
> /// ```py
> /// def hello_world():
> ///     print('Hello')
> ///     print('World')
> /// ```
> fn hello_world() {
>     println!("Hello");
>     println!("World");
> }
> ```
```

## Example

```sh
$ ./main
+ cargo run --example example
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/example`
Token(Root)@0..69
  Token(Text)@0..18 "def hello_world():"
  Newline@18..19 "\n"
  Indent@19..23 "    "
  Token(Text)@23..37 "print('Hello')"
  Newline@37..38 "\n"
  Token(Text)@38..52 "print('World')"
  Newline@52..53 "\n"
  Dedent@53..53 ""
  Indent@53..55 "# "
  Token(Text)@55..68 "hello_world()"
  Newline@68..69 "\n"
  Dedent@69..69 ""

def hello_world():
    print('Hello')
    print('World')
# hello_world()
```

## Details

- Existing `Syntax`es can be wrapped in a `Syntax2D<S>` enum which adds `Indent`/`Dedent`/`Newline` variants
  - Rust's niche optimizations make this abstraction cheap
- Functionality is provided for extracting text from "red" `ResolvedNode`s which handles indentation 
