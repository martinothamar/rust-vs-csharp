pub mod app {
    pub struct Person {
        first_name: String,
        last_name: String,
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

        pub fn get_first_name(&self) -> &str {
            &self.first_name
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
