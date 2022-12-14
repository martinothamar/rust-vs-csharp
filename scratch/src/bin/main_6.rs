use app::Fruit;

pub mod app {
    pub trait Fruit {
        fn calories_per_100_g() -> u32 where Self: Sized;
        fn weight(&self) -> u32;
        fn calories(&self) -> u32;
    }

    pub struct Banana(pub u32);

    impl Fruit for Banana {
        fn calories_per_100_g() -> u32 {
            89
        }

        fn weight(&self) -> u32 {
            self.0
        }

        fn calories(&self) -> u32 {
            Self::calories_per_100_g() * self.0 / 100
        }
    }
}

fn main() {
    let fruit = app::Banana(120);
    let calories = fruit.calories();
    println!("{}", calories);
}
