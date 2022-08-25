use std::{thread, sync::Arc};

fn main() {
    let v = Arc::new(vec![1, 2, 3]);

    let v1 = v.clone();
    let handle1 = thread::spawn(move || {
        println!("[1] Here's a vector: {:?}", v1);
    });

    let v2 = v.clone();
    let handle2 = thread::spawn(move || {
        println!("[2] Here's a vector: {:?}", v2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}