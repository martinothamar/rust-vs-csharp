use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3]; // Growable list, contents on heap
    let arr1 = [1, 2, 3]; // Fixed size array, on stack
    let arr2 = [0; 3];    // Fixed size array, 0 unitialized, on stack
    let map = HashMap::from([("n1", 1)]); // Hashmap, on heap

    log(&mut vec.iter());
    log(&mut arr1.iter());
    log(&mut arr2.iter());
    log(&mut map.iter());
}

fn log<T>(coll: &mut dyn Iterator<Item = T>) where T: Debug {
    print!("Logging for collection of {}: ", std::any::type_name::<T>());
    for item in coll {
        print!("{:?} ", item)
    }
    print!("\n");
}