use std::{thread, sync::{Mutex, Arc}};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));

    let v1 = v.clone();
    let handle1 = thread::spawn(move || {
        let mut v = v1.lock().unwrap();
        v.push(4);
        println!("[1] Here's a vector: {:?}", v);
    });

    let v2 = v.clone();
    let handle2 = thread::spawn(move || {
        let mut v = v2.lock().unwrap();
        v.push(5);
        println!("[2] Here's a vector: {:?}", v);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}