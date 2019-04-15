pub fn simple_borrow_test() {
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = s1;
    //    If we use s1,it will report a error;
    //    println!("{} has moved to s2",s1);
    //    error type: value used here after move
    println!("{} has moved to s2", s2);

    let mut s3 = String::from("hello");

    {
        let mut s4 = &s3; // &s3 是一个不可变引用
                          // &s3 is immutable, we can't change s3 value by s4.
                          // although s4 is mutable, it mean s4 could point to another &str,i.e:&s8
        println!("s3 value:{}", s3);
        println!("s4 value:{}", s4);
    }
    {
        let s4 = &mut s3; // &mut s3 是一个可变引用
                          // &mut s3 is mutable, we could change s3 value by s4.
                          // s4 is mutable, it mean s4 could point to another &str,i.e:&s8
        s4.push_str("_ownership");
        println!("s4 value:{}", s4);

        // we can't use it because s3 has been borrowed as mutable
        // println!("s3 value:{}",s3);
    }
    //borrow will return
    println!("s3 value:{}", s3);
    {
        let mut s4 = s3;
        s4.push_str("_ownership");
        // println!("s3 value:{}", s3); s3 has been borrowed
        // let s5 = s3;   (value used here after move,s3 has been borrowed)
        println!("s4 value:{}", s4);
    }

    let mut a = [1, 2, 3, 4, 5];
    {
        let slice = &mut a[1..3];
        slice[0] = 100;
    }
    println!("a[1] value: {}", a[1]);

    println!("{}", "-------------------------------");
}

pub fn ref_test() {
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{} ,{} reference test", s1, s2);
    println!("{}", "-------------------------------");
}

pub fn clone_copy_test() {
    //Cloning is an explicit action, x.clone().
    println!("{}", "-------------------------------");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} ,{} test clone", s1, s2);

    //Copies happen implicitly, for example as part of an assignment y = x
    let mut x = 5;
    let mut y = x;
    println!("x value is:{}", x);
    println!("y value is:{}", y);
    x = 3;
    y = 2;
    println!("x value is:{}", x);
    println!("y value is:{}", y);
    println!("{}", "-------------------------------");

    //Clone is a supertrait of Copy, so everything which is Copy must also implement Clone
}

pub fn move_test() {
    println!("{}", "---------------move_test start----------------");
    let mut s3 = String::from("hello");
    {
        let mut s4 = s3;
        s4.push_str("_ownership")
    }
    // It will return error: value used here after move
    // println!("s3 value:{}", s3);
    // move will not return
    println!("{}", "---------------move_test end  ----------------");
}
