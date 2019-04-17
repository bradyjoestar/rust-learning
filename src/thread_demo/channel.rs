use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn thread_channel_test(){
    println!(
        "{}",
        "----------------------thread_channel_test start----------------------"
    );

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    //通道的接收端有两个有用的方法：recv 和 try_recv。
    // 这里，我们使用了 recv，它是 receive 的缩写。
    // 这个方法会阻塞主线程执行直到从通道中接收一个值
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn thread_channel_test2(){
    /*
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    It will report the following error:

    tx.send(val).unwrap();
                    --- value moved here
    println!("val is {}", val);
                          ^^^ value used here after move

    */
}

pub fn thread_channel_multi_value(){
    println!(
        "{}",
        "----------------------thread_channel_multi_value start----------------------"
    );
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn thread_channel_multi_sender(){
    println!(
        "{}",
        "----------------------thread_channel_multi_sender start----------------------"
    );
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}