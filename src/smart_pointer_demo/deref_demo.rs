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

struct DerefBox<T> {
    name: T,
}

impl<T> DerefBox<T> {
    fn new(x: T) -> DerefBox<T> {
        DerefBox { name: x }
    }
}

impl<T> Deref for DerefBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.name
    }
}

pub fn deref_simple_demo() {
    println!(
        "{}",
        "------------deref_simple_demo start-------------------"
    );
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = String::from("deref_simple_demo");
    let d = DerefBox::new(z);

    println!("value in derefbox is : {}", *d);

    let str1 = String::from("deref_string_demo");
    println!("str1 value is : {}", str1);

    //解引用强制多态（deref coercions）是 Rust 表现在函数或方法传参上的一种便利。
    // 其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用。
    // 当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生。
    // 自动发生!
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    /*
    如果 Rust 没有解引用强制多态则必须编写的代码
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    */
    let n = String::from("Rust2");
    hello(&n);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
