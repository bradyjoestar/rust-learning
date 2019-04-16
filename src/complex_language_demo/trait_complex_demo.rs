use super::trait_simple_demo::Display;
use super::trait_simple_demo::Summary;
use super::trait_simple_demo::Tweet;

use std::fmt::Display as Display_std;

pub fn notify<T: Summary + Display>(item: T) {
    println!("Breaking news! Sumarray+Display {}", item.summarize());
}

//another style
// pub fn notify(item: impl Summary + Display) {
// }

pub fn trait_complex_test() {
    println!(
        "{}",
        "------------trait complex test start-------------------"
    );
    let tweet = returns_summarizable();

    notify(tweet);
}

fn some_function<T: Display + Clone, U: Clone + Copy>(t: T, u: U) -> i32 {
    1
}

// another writing style
//fn some_function(item1:impl Display + Clone,item2:impl Clone + Copy) -> i32{
//    1
//}

fn returns_summarizable() -> impl Summary + Display {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//为了只对实现了 PartialOrd+Copy 的类型调用这些代码
// better writing style
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn trait_bound_test() {
    println!(
        "{}",
        "------------trait complex bound test start-------------------"
    );
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only for who impl Display+PartialOrd can can cmd_display method
impl<T: Display_std + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn pair_bound_test() {
    println!(
        "{}",
        "------------trait pair bound test start-------------------"
    );
    let pair = Pair::new(100, 200);
    Pair::cmp_display(&pair);
}
