use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3]; // Growable list, contents on heap
    let arr1 = [1, 2, 3]; // Fixed size array, on stack
    let arr2 = [0; 3];    // Fixed size array, 0 unitialized, on stack
    let map = HashMap::from([("n1", 1), ("n2", 2), ("n3", 3)]); // Hashmap, on heap

    log_where(&mut vec.iter(), |i| *i % 2 == 0);
    log_where(&mut arr1.iter(), |i| *i % 2 == 0);
    log_where(&mut arr2.iter(), |i| *i % 2 == 0);
    log_where(&mut map.iter(), |(_, i)| *i % 2 == 0);
}

fn log_where<T, P>(coll: &mut dyn Iterator<Item = T>, predicate: P) where T: Debug, P: FnMut(&T) -> bool {
    print!("Logging for collection of {}: ", std::any::type_name::<T>());
    for item in coll.filter(predicate) {
        print!("{:?} ", item)
    }
    print!("\n");
}