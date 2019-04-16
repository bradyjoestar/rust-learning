use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

/*
    pub enum Result<T, E> {
    /// Contains the success value
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
*/

pub fn result_test1() {
    println!("{}", "------------result test1 start-------------------");
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    file.metadata().unwrap();
}

//unwrap_or_else & map_err is 闭包
pub fn result_test2() {
    println!("{}", "------------result test2 start-------------------");
    let file = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    file.unwrap();
}

//传播错误
//还可以选择让调用者知道这个错误并决定该如何处理
pub fn propagating_test() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//传播错误
// Reuslt<String, io::Error> is a enum type
// propagating_test2 is equal to propagating_test
// 传播错误的简写：?
// 如果出现了错误，? 会提早返回整个函数并将一些 Err 值传播给调用者。 `?` = return err
// ? 只能被用于返回值类型为 Result 的函数，
// ，所以函数的返回值必须是 Result 才能与这个 return 相兼容。
pub fn propagating_test2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
