//multi return value
pub fn func_mult_return(x: i32, y: i32) -> (i32, i32) {
    println!("{}", "-------------------------------");
    println!("{}", x + y);
    println!("{}", "-------------------------------");
    (y, x)
}

//..会产生一个iterator ，它包含左边的数，排除在右边的数。
// 为了得到一个包含两端的范围的iterator，使用...符号
pub fn for_func() {
    println!("{}", "-------------------------------");
    let a = [2, 4, 6, 8, 10];
    for element in a.iter() {
        println!("the value of loop1 is: {}", element);
    }

    // a[1..4] is a slice
    for element in a[1..4].iter() {
        println!("the value of loop2 is: {}", element);
    }

    // (a[1],a[1]+1,a[1]+2,...a[4]-1,a[4])
    // . has a highest priority , so we should add `()`
    for number in (a[1]..=a[4]).rev() {
        println!("the value of loop3 is: {}!", number);
    }

    // 1,2,3,4
    for number in (1..4).rev() {
        println!("the value of loop4 is: {}!", number);
    }

    for number in a[1]..=a[4] {
        println!("the value of loop5 is: {}!", number);
    }

    for number in 1..4 {
        println!("the value of loop6 is: {}!", number);
    }

    println!("{}", "-------------------------------");
}
