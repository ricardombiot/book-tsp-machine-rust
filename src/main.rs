pub mod tsp {
    pub mod utils;
    pub mod machine;
    pub mod actions {
        pub mod database_actions;
        pub mod action;
    }
}
//mod tsp;

#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
}
