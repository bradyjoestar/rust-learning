use module3::blah;
use module1;
use module2;


pub fn doit() {
  let value = fmt!(400);
  println!("{:?}", value);


  println!("module4");
  module1::blah::doit();
  module2::blah::doit();
  blah::doit();
}
