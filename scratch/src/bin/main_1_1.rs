pub mod app {
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }
}

fn main() {
    // Allokeres på heap
    // '_person' er en Box<>, som er en smart pointer til heap
    // deallokeres på deterministisk måte
    let _person = Box::new(app::Person { 
        first_name: String::from("Martin"), 
        last_name: String::from("Othamar") 
    });
}
