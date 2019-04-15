use tiwenbin::ownership_demo::simple_ownership_demo;

#[macro_use]
extern crate tiwenbin;

fn main() {
    let value = fmt!(100);
    println!("{}", value);

    simple_ownership_demo::simple_test();

    simple_ownership_demo::ref_test();

    simple_ownership_demo::clone_copy_test();
}
