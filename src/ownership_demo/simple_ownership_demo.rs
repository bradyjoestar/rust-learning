pub fn simple_test() {
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} has moved to s2", s2);
    println!("{}", "-------------------------------");
    //    If we use s1,it will report a error;
    //    println!("{} has moved to s2",s1);
    //    error type: value used here after move

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} ,{} test clone", s1, s2);
    println!("{}", "-------------------------------");
}
