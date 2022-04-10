
use crate::tsp::utils::alias::{Weight, Color, Km};
use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::pathset::readers::path_reader::PathSolutionReader;


pub fn run_hal_machine_dode(){
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


pub fn run_simple_graph(){
    let n = 4 as Color;
    let b_max = 4 as Km;
    let weight = 1 as Weight;

    let mut graf = Grafo::new(n);

    graf.add_bidirectional(0 as Color, 1 as Color, weight);
    graf.add_bidirectional(0 as Color, 3 as Color, weight);
    graf.add_bidirectional(1 as Color, 2 as Color, weight);
    graf.add_bidirectional(1 as Color, 3 as Color, weight);
    graf.add_bidirectional(2 as Color, 3 as Color, weight);


    let color_origin = 0;
    let mut machine = HamiltonianMachine::new(graf, color_origin);

    machine.execute();

    let graph = machine.get_one_solution_graph();
    assert!(graph.is_some());

    let graph = graph.unwrap();
    
    let _res = graph.to_png("simple".to_string(), None);
    let path = PathSolutionReader::read(n, b_max, &graph);

    println!("Solution Path: {:?}",path.route());
}
