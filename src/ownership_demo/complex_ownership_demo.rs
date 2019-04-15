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
    let mut s = String::from("slice");
    let mut slice = &s[0..3];
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
