//multi return value
pub fn func_mult_return(x: i32, y: i32) -> (i32, i32) {
    println!("{}", "-------------------------------");
    println!("{}", x + y);
    println!("{}", "-------------------------------");
    (y, x)
}

pub fn for_func() {
    println!("{}", "-------------------------------");
    let a = [2, 4, 6, 8, 10];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (a[1]..a[4]).rev() {
        println!("{}!", number);
    }

    for number in (a[1]..a[4]).rev() {
        println!("{}!", number);
    }
    println!("{}", "-------------------------------");
}
