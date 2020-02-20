use std::collections::HashMap;

use crate::concurrency::play_concurrency::{multi_thread_with_channel, simple_thread, thread_await_example, thread_with_arc, thread_with_arc_share, thread_with_channel, thread_with_move};
use crate::file::zip_file::read_zip_file;

mod concurrency;
mod file;

fn main() {
    //simple_thread();

    //thread_await_example()

    //thread_with_move();

    //thread_with_channel();

    //multi_thread_with_channel();

    //thread_with_arc();

    //thread_with_arc_share();

    read_zip_file(String::from("/Users/lbt05/test/rustOTP/test.zip"));
    unpack();
}

fn unpack() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.get(&1).map(|x| println!("{}", x));
    let value = map.get(&1);
    println!("{}", value.unwrap_or(&3));

    let data = vec![1, 2, 3];
    let v = data.get(10).unwrap_or(&10);
    println!("{}", v);
}
