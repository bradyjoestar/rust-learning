pub fn simple_test() {
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} has moved to s2", s2);
    println!("{}", "-------------------------------");
    //    If we use s1,it will report a error;
    //    println!("{} has moved to s2",s1);
    //    error type: value used here after move
}


pub fn ref_test(){
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{} ,{} reference test", s1, s2);
    println!("{}", "-------------------------------");
}

pub fn clone_copy_test(){
    //Cloning is an explicit action, x.clone().
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} ,{} test clone", s1, s2);

    //Copies happen implicitly, for example as part of an assignment y = x
    let mut x = 5;
    let mut y = x;
    println!("x value is:{}",x);
    println!("y value is:{}",y);
    x = 3;
    y = 2;
    println!("x value is:{}",x);
    println!("y value is:{}",y);
    println!("{}", "-------------------------------");

    //Clone is a supertrait of Copy, so everything which is Copy must also implement Clone
}