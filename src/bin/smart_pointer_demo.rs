use rustlearn::smart_pointer_demo::{box_demo, deref_demo, drop_demo};

extern crate rustlearn;

fn main() {
    println!("{}", "smart_pointer_demo");

    box_demo::box_simple_test();

    box_demo::box_recursive_test();

    deref_demo::deref_simple_demo();

    drop_demo::drop_simple_test();

    drop_demo::drop_defore_main_test();
}
