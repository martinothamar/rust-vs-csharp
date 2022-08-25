pub mod app {
    use std::rc::Rc;

    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }

    impl Person {
        pub fn new(first_name: &str, last_name: &str) -> Rc<Self> {
            Rc::new(Self { 
                first_name: String::from(first_name), 
                last_name: String::from(last_name) 
            })
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
    // person.first_name = String::from("Kjell");
    // person.first_name.push_str("Pell");
}
