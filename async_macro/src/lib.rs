extern crate proc_macro;

use proc_macro2::{Span, Ident};
use quote::quote;

#[proc_macro_attribute]
pub fn async_route(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // convert original TokenStream into proc_macro2 TokenStream
    let input = proc_macro2::TokenStream::from(item.clone());
    // output TokenStream; will contain original function and new _async function
    let mut output: proc_macro2::TokenStream = { input.clone() };


    // convert stream into iter and get its 1th element (fn name)
    let looky_stream = item.clone();
    let mut looky_iter = looky_stream.into_iter();
    
    
    let f_name_tree = looky_iter.nth(1).unwrap();
    
    let mut f_signature_tree_next = looky_iter.next();
    let mut f_signature_stream = proc_macro::TokenStream::new();
    
    // iteratively build rest of function as a TokenStream
    while f_signature_tree_next.is_some() {
        // convert next iter item to TokenStream
        let temp_stream = proc_macro::TokenStream::from(f_signature_tree_next.clone().unwrap());

        // add "next" TokenStream to signature_stream
        f_signature_stream.extend(temp_stream);

        f_signature_tree_next = looky_iter.next();
    }
    // now we have the rest of the function signature & body in #f_signature_stream
    // convert proc_macro::TokenStream into proc_macro2::TokenStream
    let f_signature_stream2 = proc_macro2::TokenStream::from(f_signature_stream);

    // convert original fn name into an ident
    let f_name = format!("{}", f_name_tree);
    let ident = Ident::new(&f_name, Span::call_site());

    // append `_async` to ident, get eaten by new ident
    let new_ident = Ident::new(&format!("{}_async", ident), Span::call_site());
    println!("new_ident: {}", new_ident);

    // define output syntax; compile into TokenStream
    let new_f = quote! {
        // TODO: replace T & x with function signatures from OG TokenStream
        fn #new_ident #f_signature_stream2
    };

    // convert proc_macro2 TokenStream into OG TokenStream
    let new_f_stream = proc_macro::TokenStream::from(new_f);

    // append new TokenStream (containing the async version of our fn)
    // to our old output var
    output.extend::<proc_macro2::TokenStream>(new_f_stream.into());

    // finally, convert output back into OG TokenStream format and return
    proc_macro::TokenStream::from(output)
}
