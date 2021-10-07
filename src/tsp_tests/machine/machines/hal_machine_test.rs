use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::utils::alias::{Color, Weight, Km};

use crate::tsp::pathset::readers::path_reader::PathSolutionReader;

#[test]
pub fn test_hal_machine(){
    let n = 4 as Color;
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

    let path = PathSolutionReader::read(n, b_max, &graph);

    println!("Solution Path: {:?}",path.route());
}