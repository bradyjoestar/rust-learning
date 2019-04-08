#[macro_use]
mod utils;

// sequence is very important!
// If we want to use macro of utils in module1
// we must put it below of utils defintion.

pub mod module1;
pub mod module2;
pub mod module3;

pub fn lib(){
    say_hello!();
}