pub mod app {
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }

    impl Person {
        pub fn new(first_name: &str, last_name: &str) -> Self {
            Self { first_name: String::from(first_name), last_name: String::from(last_name) }
        }

        pub fn get_full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}

fn main() {
    let person = app::Person::new("Martin", "Othamar");
    println!("{}", person.get_full_name());
}
