// use proc_macro::TokenStream;
//
// #[proc_macro]
// pub fn timer(input: TokenStream) -> TokenStream {
//     let start_time = std::time::Instant::now();
//     println!("{:?}", input);
//     let duration = start_time.elapsed();
//     println!("The calling of this method is: {:?}", duration);
//     input
// }

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro]
pub fn simple_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let function_name = &input.sig.ident;

    let expanded = quote! {
        #input

        fn main() {
            println!("Calling function: {}...", stringify!(#function_name));
            #function_name();
        }
    };

    expanded.into()
}
