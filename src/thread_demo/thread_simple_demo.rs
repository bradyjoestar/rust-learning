use std::thread;
use std::time::Duration;

pub fn thread_simple_demo1() {
    //spawn create a thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread1!", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("thread 1 finished")
    });

    handle.join().unwrap();

    for i in 1..5 {
        // run in the main thread of caller
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn thread_simple_demo2() {
    println!(
        "{}",
        "----------------------thread_simple_demo2 start----------------------"
    );
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread2!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("{}", "thread2 finished")
}

pub fn thread_move_demo3() {
    println!(
        "{}",
        "----------------------thread_move_demo3 start----------------------"
    );
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn thread_string_demo() {
    println!(
        "{}",
        "----------------------thread_string_demo start----------------------"
    );
    /*
    这段代码逻辑不正确，无法通过编译
    这样判断是合理的: 防止一切可能引发的线程安全问题
    由于A已经被移动进入闭包中
    let a;
    let handle = thread::spawn(move ||{
        a = String::from("wenbin");
    });
    handle.join().unwrap();
    println!("a value is {}",a);
    */


    /*
    另外一个例子，这段逻辑是正确的，但是还是通不过编译
    因为rust认为它有未初始化的风险
    let a;
    let b = 1;
    {
        if b == 1{
            a = String::from("thread_string_demo");
        }else{

        }
    }
    println!("{}",a);
    */

    let _a;
    {
        _a = String::from("string_demo");
    }
    println!("passed!");
}
