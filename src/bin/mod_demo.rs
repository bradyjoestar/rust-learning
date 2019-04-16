use rustlearn::mod_demo::module3::blah2;
use rustlearn::mod_demo::module3::module4;

#[macro_use]
extern crate rustlearn;

fn main() {
    let value = fmt!(500);
    println!("{:?}", value);
    rustlearn::lib();
    module4::blah::doit();
    say_hello!();

    println!("{}", "blah2Doit start");
    blah2::blah2_doit();
}
