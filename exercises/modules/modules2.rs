// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

pub mod delicious_snacks {
    pub use self::fruits as fruit;
    pub use self::veggies as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit::PEAR,
        delicious_snacks::veggie::CUCUMBER
    );
}
