use module3::blah;
use module3::module4::blah as blah4;

pub fn blah2Doit(){
    println!("{}","blah2");
    blah4::doit();
}