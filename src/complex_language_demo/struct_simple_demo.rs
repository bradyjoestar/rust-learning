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

struct Rectangle {
    width: u32,
    height: u32,
}

//方法语法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //关联函数  允许在 impl 块中定义 不 以 self 作为参数的函数
    // 用作返回一个结构体新实例的构造函数
    fn from(width: u32, weight: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: weight,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn compute_area_test() {
    println!(
        "{}",
        "------------compute area demo start-------------------"
    );
    let rect1 = Rectangle::from(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "{}",
        "------------compute area demo end  -------------------"
    );
}
