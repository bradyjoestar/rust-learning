use tiwenbin::module3::module4;
use tiwenbin::module3::blah2;

#[macro_use]
extern crate tiwenbin;

fn main() {
  let value = fmt!(500);
  println!("{:?}", value);
  tiwenbin::lib();
  module4::blah::doit();
  say_hello!();

  println!("{}","blah2Doit start");
  blah2::blah2Doit();
}
