use std::collections::HashMap;

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
    row.push(SpreadsheetCell::Int(20));
}

pub fn vec_push_test() {
    println!("{}", "------------vec push start-------------------");
    let mut str_row = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    let s = String::from("yellow");
    str_row.push(s); // s has been moved and we can't use it.

    //this cause:value used here after move
    //s.push_str("vec_push");
}

pub fn hashmap_test() {
    println!("{}", "------------hashmap start-------------------");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    //只在键没有对应值时插入,grammar candy
    scores.entry(String::from("Blue")).or_insert(500);
    scores.insert(String::from("Yellow"), 50);

    //对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    scores.insert(field_name, 200);
    // this will cause value used here after move error
    //println!("{}",field_name);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        None => println!("didnt existed"),
        Some(point) => println!("existed,value is {}", *point),
    }
}
