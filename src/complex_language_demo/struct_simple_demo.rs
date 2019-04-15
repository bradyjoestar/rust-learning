struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn build_demo() {
    println!("{}", "------------build demo start-------------------");
    let mut user = build_user(String::from("xxx@gmail.com"), String::from("XXX"));
    println!("{}", user.email);
    user.email.push_str("@foxmail");
    println!("{}", user.email);
    println!("{}", "------------build demo end  -------------------");
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
