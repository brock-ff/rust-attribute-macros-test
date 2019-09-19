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
    // convert original TokenStream into proc_macro2 TokenStream
    let input = proc_macro2::TokenStream::from(item.clone());
    // output TokenStream; will contain original function and new _async function
    let mut output: proc_macro2::TokenStream = { input };
    println!("{}", item);

    // convert stream into iter and get its 1th element (fn name)
    let looky_stream = item.clone();
    let mut looky_iter = looky_stream.into_iter();
    let f_token_tree = looky_iter.nth(1).unwrap();

    // convert original fn name into an ident
    let f_name = format!("{}", f_token_tree);
    let ident = Ident::new(&f_name, Span::call_site());

    // append `_async` to ident, get eaten by new ident
    let new_ident = Ident::new(&format!("{}_async", ident), Span::call_site());
    println!("new_ident: {}", new_ident);

    // define output syntax; compile into TokenStream
    let new_f = quote! {
        // TODO: replace T & x with function signatures from OG TokenStream
        fn #new_ident<T>(x: T) where T: std::fmt::Display {
            println!("\"async\" thing triggered\n{}", x);
        }
    };

    // convert proc_macro2 TokenStream into OG TokenStream
    let new_f_stream = proc_macro::TokenStream::from(new_f);

    // append new TokenStream (containing the async version of our fn)
    // to our old output var
    output.extend::<proc_macro2::TokenStream>(new_f_stream.into());

    // finally, convert output back into OG TokenStream format and return
    proc_macro::TokenStream::from(output)
}
