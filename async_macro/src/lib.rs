extern crate proc_macro;

use proc_macro2::{Span, Ident};
use quote::quote;

#[proc_macro_attribute]
pub fn async_route(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // convert original TokenStream into proc_macro2 TokenStream
    let input = proc_macro2::TokenStream::from(item.clone());
    // output TokenStream; will contain original function and new _async function
    let mut output: proc_macro2::TokenStream = { input.clone() };

    // first, clone iterator and call next() on it until it returns None to get its length
    // then we'll use that to identify the last element; the function body
    // note: we have to use length because TokenStream does not implement any equality trait
    let mut counter_iter = item.clone().into_iter();
    let mut i = 0;
    while counter_iter.next().is_some() {
        i += 1;
    }

    // convert stream into iter and get its 1th element (fn name)
    let mut looky_iter = item.clone().into_iter();
    let f_name_tree = looky_iter.nth(1).unwrap();
    
    // build a token stream containing the function signature
    let mut f_signature_tree_next = looky_iter.next();
    let mut f_signature_stream = proc_macro::TokenStream::new();
    
    // iterate up to (but not including) the last element
    // and remember, we consumed the first two elements from the iter to get the name,
    // so we need (i - 3) instead of (i - 1) to stop before the last element
    let mut j = 0;
    while j < i - 3 {
        // convert next iter item to TokenStream
        let temp_stream = proc_macro::TokenStream::from(f_signature_tree_next.clone().unwrap());

        // add "next" TokenStream to signature_stream
        f_signature_stream.extend(temp_stream);

        // consume next iter element
        f_signature_tree_next = looky_iter.next();
        j += 1;
    }
    
    // now we have the function signature in #f_signature_stream
    // convert proc_macro::TokenStream into proc_macro2::TokenStream
    let f_signature_stream2 = proc_macro2::TokenStream::from(f_signature_stream);

    // create TokenStream from last iter element (fn body)
    // last element is consumed by f_signature_tree_next 
    // in the last iteration of preceding loop
    let f_body_tree = f_signature_tree_next;
    let f_body_stream = proc_macro::TokenStream::from(f_body_tree.unwrap());
    
    // now we have the function body in #f_body_stream
    // convert proc_macro::TokenStream into proc_macro2::TokenStream
    let f_body_stream2 = proc_macro2::TokenStream::from(f_body_stream);

    // convert original fn name into an ident
    let f_name = format!("{}", f_name_tree);
    let ident = Ident::new(&f_name, Span::call_site());

    // append `_async` to ident, get eaten by new ident
    let new_ident = Ident::new(&format!("{}_async", ident), Span::call_site());
    println!("procedurally generated new function: {}", new_ident);

    // define output body; compile into TokenStream with quote!
    let new_f = quote! {
        fn #new_ident #f_signature_stream2 {
            println!("Do this, then do whatever else it is that this function does...");
            #f_body_stream2
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
