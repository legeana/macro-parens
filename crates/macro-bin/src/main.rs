use macro_lib_derive::my_macro_impl;

macro_rules! my_macro {
    ($arg:expr) => {
        my_macro_impl!($arg)
    };
}

fn main() {
    println!("{}", my_macro!(1 + 1));
}
