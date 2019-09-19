extern crate async_macro;
use async_macro::*;

#[async_route]
fn some_post(req: String) {
    println!("{}", req);
}

macro_rules! do_function {
    ($f: expr, $($x: expr),*) => {
        {$f($($x),*)};
    };
}

fn main() {
    println!("Hello, world!");
    let req = "OK".to_owned();
    do_function!(some_post_async, req);
    let req = "OK".to_owned();
    do_function!(some_post, req);
}
