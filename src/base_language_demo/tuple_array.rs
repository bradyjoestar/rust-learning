pub fn tuple() {
    let tup = (500, 6.4, 1);

    let (_x, mut _y, _z) = tup;

    println!("The value of mut y is: {}", _y);

    _y = 100.1;

    println!("The value of mut y is: {}", _y);

    //mut
    let mut tup2 = (500, 5.2, 3);

    tup2.1 = 4.2;

    println!("{}", tup2.1)
}

// Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
pub fn array() {
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    let mut b = [6, 7, 8, 9, 10];
    b[2] = 180;
    println!("The value of b[2] is: {}", b[2])
}
