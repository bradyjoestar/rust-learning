use tiwenbin::base_language_demo::{tuple_array,function};

#[macro_use]
extern crate tiwenbin;

fn main(){
    let value = fmt!(100);
    println!("{}",value);

    tuple_array::tuple();

    tuple_array::array();

    let func_result = function::func_test(100,200);

    println!("func result value 1: {}",func_result.0);
    println!("func result value 2: {}",func_result.1);
}