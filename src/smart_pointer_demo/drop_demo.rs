struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_simple_test() {
    println!(
        "{}",
        "------------drop_simple_test start-------------------"
    );
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

//通过 std::mem::drop 提早丢弃值
pub fn drop_defore_main_test() {
    println!(
        "{}",
        "------------drop_defore_main_test start-------------------"
    );
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
