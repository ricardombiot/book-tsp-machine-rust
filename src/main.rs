pub mod tsp {
    pub mod utils;
    pub mod machine {
        pub mod components {
            pub mod timeline;
            pub mod graf;
        }

        pub mod machines {
            pub mod hal_machine;
        }
    }
    pub mod actions {
        pub mod database_actions;
        pub mod action;
        pub mod execute_actions;
        pub mod table_graph_by_length;
        pub mod table_actions;
    }
    pub mod pathset;
}
//mod tsp;

#[cfg(test)]
mod tsp_tests;

use crate::tsp::utils::alias::{Weight, Color};
use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
fn main() {
    //println!("Hello, world!");


    let n = 12 as Color;
    let weight = 1 as Weight;
    let g = Grafo::gen_complete(n, weight);

    //println!("{:#?}", g);

    let color_origin = 0;
    let mut machine = HamiltonianMachine::new(g, color_origin);

    machine.execute();
}
