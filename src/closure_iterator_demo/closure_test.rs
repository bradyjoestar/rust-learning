use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn closure_simple_test() {
    println!(
        "{}",
        "------------closure_simple_test start-------------------"
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    let closure_test = |number1, number2| {
        println!("find biggest number...");
        if number1 > number2 {
            number1
        } else {
            number2
        }
    };

    println!("{}", expensive_closure(20));
    println!("{}", closure_test(20, 50));
}

pub fn closure_complex_test() {
    println!(
        "{}",
        "------------closure_complex_test start-------------------"
    );

    let closure_test = |str1: &str, str2: &str| -> &str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    };

    let string1 = String::from("closure");
    let string2 = String::from("demo");

    /*
    It will meet the following error if we want to use it
    "cannot infer an appropriate lifetime due to conflicting requirements"

    And this error couldn't be solved
    */
    //println!("{}",closure_test(string1.as_str(),string2.as_str()))
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => {
                println!("we have it");
                return *v;
            }
            None => {
                println!("{}", "not existed");
            }
        }
        println!("insert couldn't be include in get");
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        let v = (self.calculation)(arg);
        self.value.insert(arg, v);
        v
    }
}

pub fn generic_closure_test() {
    println!(
        "{}",
        "------------generic_closure_test start-------------------"
    );
    let mut expensive_result = Cacher::new(|num| num);

    let intensity = 20;
    let random_number = 1;

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity + 1));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

//闭包会捕获其环境
pub fn closure_environment_demo() {
    println!(
        "{}",
        "------------closure_environment_demo start-------------------"
    );
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    println!("{}", equal_to_x(y));

    /*
    当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销，
    在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。
    因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

    闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。
    */
}

pub fn closure_trait_demo() {
    println!(
        "{}",
        "------------closure_trait_demo start-------------------"
    );
    //FnMut 获取可变的借用值所以可以改变其环境
    //Fn 从其环境获取不可变的借用值
    /*
        It will report:
        value moved (into closure) here

    let x = vec![1, 2, 3];

    let equal_to_x = move |z:Vec<i32>| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    */
}
