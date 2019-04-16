use rustlearn::closure_iterator_demo::closure_test;
use rustlearn::closure_iterator_demo::iterator_demo;

extern crate rustlearn;

fn main() {
    println!("{}", "closure_iterator_demo");

    iterator_demo::iterator_simple_test();

    closure_test::closure_simple_test();

    closure_test::closure_complex_test();

    closure_test::generic_closure_test();

    closure_test::closure_environment_demo();
}
