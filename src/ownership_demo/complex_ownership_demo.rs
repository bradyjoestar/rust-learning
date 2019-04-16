pub fn borrow_func_test() {
    println!(
        "{}",
        "-----------complex borrow func test start--------------------"
    );

    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    println!(
        "{}",
        "-----------complex borrow func test end--------------------"
    );
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("_ownership");
    s.len()
}

pub fn slice_test() {
    println!("{}", "-----------slice test start--------------------");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello_slice value is: {}", hello);
    println!("world_slice value is: {}", world);

    // string slice is unmutable
    let s = String::from("slice");
    let slice = &s[0..3];
    println!("slice value is: {}", slice);

    // array slice is mutable
    let mut a = [1, 2, 3, 4, 5];
    println!("a[1] value is:{}", a[1]);
    {
        let slice = &mut a[1..3];

        slice[0] = 1000;

        println!("slice[0] value is:{}", slice[0]);
    }

    println!("a[1] value is:{}", a[1]);

    println!("{}", "-----------slice test end  --------------------");
}

// &str
pub fn andstr_test() {
    println!("{}", "----------and str end  --------------------");
    //This will not report error
    let mut s = "generic";

    let mut s4 = s;

    let mut s5 = s;

    println!("{},{},{}", s4, s, s5);

    s = "_generic";

    s4 = "gene_ric";

    s5 = "generic_";

    println!("{},{},{}", s4, s, s5);

    let strtest = String::from("andstr");

    let strtest2 = &strtest;

    let strtest3 = strtest2;

    let strtest4 = strtest2;

    println!("{},{},{},{}", strtest, strtest2, strtest3, strtest4);

    let mut a = 5;

    let c = &mut a;

    *c = 4;

    // cannot borrow `a` as immutable because it is also borrowed as mutable
    //println!("{},{}", a, c)

    let a = 10;

    let c = &a;

    //cannot assign to `a` because it is borrowed
    //assignment to borrowed `a` occurs here
    // a = 100;
}
