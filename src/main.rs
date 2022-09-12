pub mod tsp {
    pub mod utils;
    pub mod machine {
        pub mod components {
            pub mod timeline;
            pub mod graf;
        }

        pub mod machines {
            pub mod hal_machine;
            pub mod tsp_machine;
        }
    }
    pub mod actions {
        pub mod database_actions;
        pub mod action;
        pub mod execute_actions;
        pub mod table_graph_by_length;
        pub mod table_actions;
        
        pub mod table_controller;
        pub mod db_controller;
    }
    pub mod pathset;
}

#[cfg(test)]
mod tsp_tests;
mod app {
    pub mod main; 
    mod examples;
    mod bench;
}

fn main() {
    app::main::run();
}
