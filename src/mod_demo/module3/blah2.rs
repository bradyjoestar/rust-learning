use crate::mod_demo::module3::module4::blah as blah4;

pub fn blah2_doit() {
    println!("{}", "blah2");
    blah4::doit();
}
