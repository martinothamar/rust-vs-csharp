pub mod app {
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }

    impl Person {
        pub fn get_full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}

fn main() {
    let person = app::Person { 
        first_name: String::from("Martin"), 
        last_name: String::from("Othamar") 
    };
    println!("{}", person.get_full_name());
}
