pub mod tsp {
    pub mod utils;
    pub mod machine;
    pub mod actions {
        pub mod database_actions;
        pub mod action;
    }
    pub mod pathset;
}
//mod tsp;

#[cfg(test)]
mod tsp_tests;

fn main() {
    println!("Hello, world!");
}
