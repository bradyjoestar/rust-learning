use std::thread;
use std::time::Duration;
use std::collections::HashMap;

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

    let closure_test = |number1,number2|{
        println!("find biggest number...");
        if number1>number2{
            number1
        }else{
            number2
        }
    };

    println!("{}",expensive_closure(20));
    println!("{}",closure_test(20,50));
}


pub fn closure_complex_test(){
    println!(
        "{}",
        "------------closure_complex_test start-------------------"
    );

    let closure_test = |str1:&str,str2:&str| -> &str{
        if str1.len()>str2.len(){
            str1
        }else{
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
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32,u32>,
}



impl<T> Cacher<T>
    where T: Fn(u32) -> u32
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
                return *v
            }
            None => {
                println!("{}","not existed");
            },
        }
        println!("insert couldn't be include in get");

        let v = (self.calculation)(arg);
        self.value.insert(arg,v);
        v
    }
}


pub fn generic_closure_test(){
    println!(
        "{}",
        "------------generic_closure_test start-------------------"
    );
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let intensity = 20;
    let random_number = 1;

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity+1)
        );
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
pub fn closure_environment_demo(){
    println!(
        "{}",
        "------------closure_environment_demo start-------------------"
    );
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    println!("{}",equal_to_x(y));
}