use tsp;

pub mod actions {
    mod database_actions_test;
    mod action_test;
}

pub mod machine {
    pub mod components {
        mod graf_test;
    }
}

pub mod utils {
    mod generator_ids_test;
    mod generator_node_key_test;
}

pub mod pathset {
    pub mod components {
        pub mod nodes {
            pub mod node_test;
            pub mod node_id_test;
        }

        pub mod edges {
            pub mod edge_test;
        }

        pub mod owners {
            pub mod owners_test;
        }
    }
}