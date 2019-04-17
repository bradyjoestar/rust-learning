use std::fmt::Display;
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn smart_pointer_self_define() {
    println!(
        "{}",
        "---------------smart_pointer_self_define start----------------"
    );

    let x = 5;
    let y = MyBox::new(x);
    println!("my diy smart pointer value = {}", *y);
}
