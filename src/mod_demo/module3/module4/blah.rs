use mod_demo::module1;
use mod_demo::module2;
use mod_demo::module3::blah;

pub fn doit() {
    let value = fmt!(400);
    println!("{:?}", value);

    println!("module4");
    module1::blah::doit();
    module2::blah::doit();
    blah::doit();
}
