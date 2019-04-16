use complex_language_demo::enum_demo::Message::ChangeColor;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum_test() {
    println!("{}", "------------enum demo start-------------------");
    let msg_a = Message::Quit;

    let msg_b = Message::Move { x: 10, y: 20 };

    let msg_c = Message::Write(String::from("enum_test_string"));

    let msg_d = ChangeColor(10, 20, 30);

    value_in_msg(msg_a);
    value_in_msg(msg_b);
    value_in_msg(msg_c);
    value_in_msg(msg_d);
}

fn value_in_msg(msg: Message) -> i32 {
    match msg {
        Message::Quit => 0,
        Message::Move { x, y } => x + y,
        Message::ChangeColor(x, y, z) => x + y + z,
        Message::Write(string) => {
            println!("the string in the msg is {}", string);
            100
        }
    }
}

pub fn option_test() {
    println!("{}", "------------option demo start-------------------");
    let number = plus_one(Some(5));
    match number {
        None => {
            println!("NONO");
        }
        Some(id) => println!("{}", id),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
