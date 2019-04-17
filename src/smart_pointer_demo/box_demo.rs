pub fn box_simple_test() {
    let b = Box::new(5);
    println!("b = {}", b);

    let mut b1 = Box::new([1, 2, 3, 4, 5, 6]);
    b1[0] = 100;
    println!("b1 = {}", b1[0]);

    let b2 = Box::new("box_simple_test");
    println!("b2 = {}", b2);
}

enum List {
    Cons(i32, Box<List>), //Box seems like pointer, so could compute.
    Nil,
}

use self::List::{Cons, Nil};

pub fn box_recursive_test() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
