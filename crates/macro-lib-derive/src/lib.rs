use proc_macro::Delimiter;
use proc_macro::TokenTree;

use quote::quote;

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
