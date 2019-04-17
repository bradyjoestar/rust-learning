use rustlearn::thread_demo::{channel, mutex_demo, thread_simple_demo};

extern crate rustlearn;

fn main() {
    println!(
        "{}",
        "----------------------thread_demo----------------------"
    );

    thread_simple_demo::thread_simple_demo1();

    thread_simple_demo::thread_simple_demo2();

    thread_simple_demo::thread_move_demo3();

    thread_simple_demo::thread_string_demo();

    channel::thread_channel_test();

    channel::thread_channel_test2();

    channel::thread_channel_multi_value();

    channel::thread_channel_multi_sender();

    mutex_demo::mutex_simple_demo();

    mutex_demo::mutex_thread_demo();

    mutex_demo::thread_rc_mutex_demo();

    mutex_demo::thread_arc_mutex_demo();
    //当主线程结束时，新线程也会结束，而不管其是否执行完毕
}
