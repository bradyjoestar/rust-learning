use tiwenbin::complex_language_demo::trait_simple_demo::Summary;
use tiwenbin::complex_language_demo::{
    collection_demo, enum_demo, generic_demo, panic_result_test, struct_simple_demo,
    trait_complex_demo, trait_simple_demo,
};

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}: {}", self.username, self.retweet)
    }
}

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

    panic_result_test::result_test1();

    panic_result_test::result_test2();

    panic_result_test::propagating_test().unwrap();

    panic_result_test::propagating_test2().unwrap();

    generic_demo::generic_struct();

    generic_demo::generic_method();

    trait_simple_demo::trait_simple_test();

    extern_trait();

    trait_complex_demo::trait_complex_test();

    trait_complex_demo::trait_bound_test();

    trait_complex_demo::pair_bound_test();
}

fn extern_trait() {
    println!("{}", "------------extern_trait start-------------------");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
