use std::sync::mpsc; // mutiple producer single consumer
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // join

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // channel

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();
    });
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx1.send(String::from("cookie")).unwrap();
    });

    for recv in rx {
        println!("Got: {}", recv);
    }

    // mutex

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
