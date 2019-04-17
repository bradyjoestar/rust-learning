use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_simple_demo() {
    println!(
        "{}",
        "----------------------mutex_simple_demo start----------------------"
    );
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn mutex_thread_demo() {
    /*
        let counter = Mutex::new(0);
        let mut handles = vec![];

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);

        let handle2 = thread::spawn(move || {
            let mut num2 = counter.lock().unwrap();

            *num2 += 1;
        });
        handles.push(handle2);

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());


        This will cause the following error:
        let handle = thread::spawn(move || {
       |                                ------- value moved into closure here
    24 |         let mut num = counter.lock().unwrap();
       |                       ------- variable moved due to use in closure
    ...
    30 |     let handle2 = thread::spawn(move || {
       |                                 ^^^^^^^ value used here after move
    31 |         let mut num2 = counter.lock().unwrap();
       |                        ------- use occurs due to use in closure

        counter 被移动进了 handle 所代表线程的闭包中。
        因此我们无法在第二个线程中对其调用 lock，
        并将结果储存在 num2 中时捕获counter！
        所以 Rust 告诉我们不能将 counter 的所有权移动到多个线程中。
        */
}

pub fn thread_rc_mutex_demo() {
    /*
    Rc<T> 并不能安全的在线程间共享。
    当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，
    并在每一个克隆被丢弃时减少计数。Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断

    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    */
}

//  Rust 不能避免使用 Mutex<T> 的全部逻辑错误。
//  一旦用到Mutex<T>,就会引入死锁风险
pub fn thread_arc_mutex_demo() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
