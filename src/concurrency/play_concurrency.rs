use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn simple_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn thread_await_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

pub fn thread_with_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn thread_with_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Json said Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn multi_thread_with_channel() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    let handler1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });


    let handler2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    handler1.join().unwrap();
    handler2.join().unwrap();

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn thread_with_arc() {
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

pub fn thread_with_arc_share() {
    let mut dictionary = HashMap::new();
    dictionary.insert(0, 0);
    dictionary.insert(1, 1);
    dictionary.insert(2, 4);
    dictionary.insert(3, 9);
    dictionary.insert(4, 16);
    dictionary.insert(5, 255);
    let (tx, rx) = mpsc::channel();
    let share_data = Arc::new(RwLock::new(dictionary));

    let mut handles = vec![];
    for key in 1..100 {
        let data = Arc::clone(&share_data);
        let thread_tx = tx.clone();
        let handle = thread::spawn(move || calculation_method(key, data, thread_tx));
        handles.push(handle);
    }
    for _ in 1..100 {
        println!("Result: {:?}", rx.recv());
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn calculation_method(key: i32, data: Arc<RwLock<HashMap<i32, i32, RandomState>>>, thread_tx: Sender<i32>) -> () {
    let num = data.read().unwrap();
    let v = key % 6;
    let value = num.get(&v).unwrap();
    // this clone is very important because value's ownership needs to be returned by the end of this closure, but share_result's not.
    thread_tx.send(Clone::clone(value)).unwrap();
}
