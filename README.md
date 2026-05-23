# Demonstration of rust-analyzer and rustc inconsistency

Given the following proc-macro:

```rust
#[proc_macro]
pub fn my_macro_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token = input.clone().into_iter().next().unwrap();
    match token {
        TokenTree::Group(g) => match g.delimiter() {
            // Rust Analyzer produces Groups with Parenthesis delimiter for
            // macro_rules arguments.
            Delimiter::Parenthesis => quote! {
                compile_error!("Unexpected Group/Parenthesis")
            }
            .into(),
            // Rustc produces Groups with None delimiter for macro_rules
            // arguments.
            Delimiter::None => input,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
```

And the following call site:

```rust
use macro_lib_derive::my_macro_impl;

macro_rules! my_macro {
    ($arg:expr) => {
        my_macro_impl!($arg)
    };
}

fn main() {
    println!("{}", my_macro!(1 + 1));
}
```

The following succeeds, as expected. The first token is a `Group` with a `None`
delimiter:

```sh
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/macro-bin`
2
```

Rust-analyzer unexpectedly produces the first `Group` token with a `Parenthesis`
delimiter:

```sh
rust-analyzer diagnostics .
at crate macro_bin, file /Users/liri/macro-parens/crates/macro-bin/src/main.rs: Error Ra("macro-error", Error) from LineCol { line: 15, col: 4 } to LineCol { line: 15, col: 12 }: Unexpected Group/Parenthesis
at crate macro_bin, file /Users/liri/macro-parens/crates/macro-bin/src/main.rs: Error SyntaxError from LineCol { line: 15, col: 4 } to LineCol { line: 15, col: 12 }: Syntax Error in Expansion: expected expression

diagnostic scan complete

Error: diagnostic error detected

Stack backtrace:
   0: <std::backtrace::Backtrace>::create
   1: <anyhow::Error>::msg::<&str>
   2: std::sys::backtrace::__rust_begin_short_backtrace::<<stdx::thread::Builder>::spawn<<rust_analyzer::cli::flags::Diagnostics>::run::{closure#0}, core::result::Result<(), anyhow::Error>>::{closure#0}, core::result::Result<(), anyhow::Error>>
   3: <std::thread::lifecycle::spawn_unchecked<<stdx::thread::Builder>::spawn<<rust_analyzer::cli::flags::Diagnostics>::run::{closure#0}, core::result::Result<(), anyhow::Error>>::{closure#0}, core::result::Result<(), anyhow::Error>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
   4: <std::sys::thread::unix::Thread>::new::thread_start
   5: __pthread_cond_wait
```
