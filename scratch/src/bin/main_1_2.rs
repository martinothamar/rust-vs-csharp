pub mod app {
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }
}

fn main() {
    // Allokeres på stack,
    // Deallokeres automatisk via stack frame pointer
    let _person = app::Person { 
        first_name: String::from("Martin"), 
        last_name: String::from("Othamar") 
    };
}
