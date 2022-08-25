pub mod app {
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }

    impl Person {
        pub fn new(first_name: &str, last_name: &str) -> Self {
            Self { 
                first_name: String::from(first_name), 
                last_name: String::from(last_name) 
            }
        }

        pub fn get_full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}

fn main() {
    let mut person = app::Person::new(
        "Martin", 
        "Othamar"
    );

    // Pass by value - ~48 bytes copied
    log_by_value(person);
    // Pass by value by exclusive mutable reference - 8 bytes copied
    log_by_mutable_reference(&mut person);
    // Pass by value by shared reference - 8 bytes copied
    log_by_shared_reference(&person);

    // String = Vec<u8> = { ptr: *mut T,  cap: usize, len: usize }
}

fn log_by_value(person: app::Person) {
    println!("{}", person.get_full_name());
}

fn log_by_mutable_reference(person: &mut app::Person) {
    println!("{}", person.get_full_name());
}

fn log_by_shared_reference(person: &app::Person) {
    println!("{}", person.get_full_name());
}
