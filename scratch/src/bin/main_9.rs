use std::{thread};

fn main() {
    let v = vec![1, 2, 3];

    let handle1 = thread::spawn(move || {
        println!("[1] Here's a vector: {:?}", v);
    });

    let handle2 = thread::spawn(move || {
        println!("[2] Here's a vector: {:?}", v);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}