use tiwenbin::complex_language_demo::{
    collection_demo, enum_demo, panic_result_test, struct_simple_demo,
};

extern crate tiwenbin;

fn main() {
    println!("{}", "complex language demo");

    struct_simple_demo::build_demo();

    struct_simple_demo::compute_area_test();

    enum_demo::enum_test();

    enum_demo::option_test();

    collection_demo::vec_simple_test();

    collection_demo::enum_vec_test();

    collection_demo::vec_push_test();

    collection_demo::hashmap_test();

    panic_result_test::test1();

    panic_result_test::test2();
}
