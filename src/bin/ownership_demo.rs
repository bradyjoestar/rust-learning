use rustlearn::ownership_demo::{complex_ownership_demo, simple_ownership_demo};

#[macro_use]
extern crate rustlearn;

fn main() {
    let value = fmt!(100);
    println!("{}", value);

    simple_ownership_demo::simple_borrow_test();

    simple_ownership_demo::clone_copy_test();

    complex_ownership_demo::borrow_func_test();

    complex_ownership_demo::slice_test();

    complex_ownership_demo::andstr_test();
}
