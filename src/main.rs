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
        
        pub mod table_controller;
        pub mod db_controller;
    }
    pub mod pathset;
}
//mod tsp;

#[cfg(test)]
mod tsp_tests;

use crate::tsp::utils::alias::{Weight, Color, Km};
use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
fn main() {
    //println!("Hello, world!");
    /*

    let n = 12 as Color;
    let weight = 1 as Weight;
    let g = Grafo::gen_complete(n, weight);

    //println!("{:#?}", g);

    let color_origin = 0;
    let mut machine = HamiltonianMachine::new(g, color_origin);

    machine.execute();*/


    let path = std::env::current_exe().unwrap();
    let path_txt = path.display().to_string();
    let path_txt = path_txt + "/test_visual/graph";
    println!("{}",path_txt);

    //test_hal_machine_complete();

    //test_hal_machine_dode();
    brench();
}

use std::time::Instant;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn brench(){

    /*let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);*/

    let mut list_times : Vec<(Color, Duration)> = Vec::new();
    let max = 15;
    for n in 4..max {
        let start = Instant::now();
        test_hal_machine_complete(n as Color);
        let elapsed = start.elapsed();
        list_times.push((n, elapsed.clone()));
        println!("Graf: {} Millis: {} ms", n, elapsed.as_millis());
    }

    println!("## BRENCHMARK ##");
    for (n, elapsed) in list_times {
        println!("Graf: {} Millis: {} ms", n, elapsed.as_millis());
    }

}

fn get_in_ms(since_the_epoch : Duration) -> u64 {
    let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    return in_ms;
}


pub fn test_hal_machine_complete(n : Color){
    //let n = 10 as Color;
    let b_max = n as Km;
    let weight = 1 as Weight;
    let g = Grafo::gen_complete(n, weight);

    //println!("{:#?}", g);

    let color_origin = 0;
    let mut machine = HamiltonianMachine::new(g, color_origin);

    machine.execute();

    let graph = machine.get_one_solution_graph();
    assert!(graph.is_some());
    //println!("{:#?}", machine);
    let graph = graph.unwrap();
    //graph.to_png("hola".to_string(), None);
    let path = PathSolutionReader::read(n, b_max, &graph);

    println!("Solution Path: {:?}",path.route());
}

fn test_hal_machine_dode(){
    let n = 20 as Color;
    let b_max = n as Km;
    let g = Grafo::gen_dodecaedro();

    //println!("{:#?}", g);

    let color_origin = 0;
    let mut machine = HamiltonianMachine::new(g, color_origin);

    machine.execute();

    let graph = machine.get_one_solution_graph();
    assert!(graph.is_some());
    //println!("{:#?}", machine);
    let graph = graph.unwrap();

    let path = PathSolutionReader::read(n, b_max, &graph);

    println!("Solution Path: {:?}",path.route());
}