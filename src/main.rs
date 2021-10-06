pub mod tsp {
    pub mod utils;
    pub mod machine {
        pub mod components {
            pub mod timeline;
            pub mod graf;
        }
    }
    pub mod actions {
        pub mod database_actions;
        pub mod action;
        pub mod execute_actions;
        pub mod table_graph_by_length;
    }
    pub mod pathset;
}
//mod tsp;

#[cfg(test)]
mod tsp_tests;

fn main() {
    println!("Hello, world!");
}
