extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::{Span, Ident};
use quote::quote;
//use syn::*;

#[proc_macro_attribute]
pub fn async_route(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(item.clone());
    let mut output: proc_macro2::TokenStream = { input };
    println!("{}", item);

    // let another = quote! {
    //     fn idk() {
    //         println!("Sweet");
    //     }
    // };

    let looky_stream = item.clone();
    // convert stream into iter and get its 1th element (fn name)
    let mut looky_iter = looky_stream.into_iter();
    let f_token_tree = looky_iter.nth(1).unwrap();
    let f_name = format!("{}", f_token_tree);
    let ident = Ident::new(&f_name, Span::call_site());

    let temp_ident = Ident::new(&format!("{}_async", ident), Span::call_site());

    println!("temp_ident: {}", temp_ident);
    let new_f = quote! {
        fn #temp_ident<T>(x: T) where T: std::fmt::Display {
            println!("async thing triggered\n{}", x);
        }
    };
    let new_f_stream = proc_macro::TokenStream::from(new_f);
    output.extend::<proc_macro2::TokenStream>(new_f_stream.into());

    proc_macro::TokenStream::from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
