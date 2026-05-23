use macro_lib_derive::my_macro_impl;

// A wrapper macro_rules macro is necessary to trigger this inconsistency.
// It appears that rustc and rust-analyzer wrap $arg in a group with a different type of delimiter:
// - rustc uses None.
// - rust-analyzer uses Parenthesis.
macro_rules! my_macro {
    ($arg:expr) => {
        my_macro_impl!($arg)
    };
}

fn main() {
    // A composite expression as my_macro argument is necessary to trigger this inconsistency.
    // Rust-analyzer will display an inline error here.
    println!("{}", my_macro!(1 + 1));
}
