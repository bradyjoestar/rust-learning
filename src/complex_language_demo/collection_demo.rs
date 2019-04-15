pub fn vec_simple_test() {
    println!("{}", "------------vec demo start-------------------");
    let mut v = vec![1, 2, 3];
    v.push(20);

    // every element pls 50  and return the pointer
    for i in &mut v {
        *i += 50;
    }

    // print all of element
    for i in &v {
        println!("{}", i);
    }

    match v.get(2) {
        //v.get return a option<T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // another create a new vec
    let mut v2: Vec<i32> = Vec::new();
    v2.push(100)
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn enum_vec_test() {
    println!("{}", "------------enum vec demo start-------------------");
    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

pub fn vec_push_test() {
    println!("{}", "------------vec push start-------------------");
    let mut str_row = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    let mut s = String::from("yellow");
    str_row.push(s);

    //this cause:value used here after move
    //s.push_str("vec_push");
}
