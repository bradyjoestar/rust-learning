use rustlearn::base_language_demo::{function, tuple_array};

#[macro_use]
extern crate rustlearn;

fn main() {
    let value = fmt!(100);
    println!("{}", value);

    tuple_array::tuple();

    tuple_array::array();

    let func_result = function::func_mult_return(100, 200);

    println!("func result value 1: {}", func_result.0);
    println!("func result value 2: {}", func_result.1);

    function::for_func();
}
