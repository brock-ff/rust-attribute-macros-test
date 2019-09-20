extern crate async_macro;
use async_macro::*;

#[async_route]
fn some_post<T, U>(a: T, b: U) -> Result<(), ()> where T: std::fmt::Display, U: std::fmt::Debug {
    println!("{}", a);
    println!("{:?}", b);
    Ok(())
}

macro_rules! do_function {
    ($f: expr, $($x: expr),*) => {
        {$f($($x),*)};
    };
}

#[derive(Debug, Clone)]
struct Test {
    f: u64,
}

fn main() {
    let a = "this is called by some_post_async".to_owned();
    let b = Test {
        f: 64,
    };
    do_function!(some_post_async, a, b.clone()).unwrap();

    let a = "this is called by some_post (synchronous)".to_owned();
    do_function!(some_post, a, b.clone()).unwrap();
}
