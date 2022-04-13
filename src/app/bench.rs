

use crate::tsp::utils::alias::{Weight, Color, Km};
use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::pathset::readers::path_reader::PathSolutionReader;

use std::time::Instant;
use std::time::Duration;

use std::fs::File;
use std::io::Write;

pub fn run_bench_to_max_n(args: &Vec<String>){
    let max_n = args[2].parse::<Color>().unwrap();
    brench(max_n);
}

pub fn brench(max_n : usize){

    let mut list_times : Vec<(Color, Duration)> = Vec::new();
    for n in 4..max_n {
        let start = Instant::now();
        test_hal_machine_complete(n as Color);
        let elapsed = start.elapsed();
        list_times.push((n, elapsed.clone()));
        println!("Graf: {} Time: {} ms", n, elapsed.as_millis());
    }

    println!("## BRENCHMARK ##");
    for (n, elapsed) in list_times {
        println!("Graf: {} Time: {} ms", n, elapsed.as_millis());
    }
}


pub fn run_bench_quick(args: &Vec<String>){
    let n = args[2].parse::<Color>().unwrap();
    let times = args[3].parse::<i32>().unwrap();
    let file_name = args[4].clone();

    brench_ntimes(n as Color,times, file_name);
}


pub fn brench_ntimes(n : Color, times : i32, file_name : String){

    let mut data = String::new();

    for iteration in 0..times {
        let start = Instant::now();
        test_hal_machine_complete(n as Color);
        let elapsed = start.elapsed();
        let report = format!("{}", elapsed.as_millis());
        
        data.push_str(&report);
        let is_end_iter = iteration == times-1;
        if !is_end_iter {
            data.push_str(&";");
        }
       

        let rest_iterations = (times-iteration) as u128;
        let estimation_ms = elapsed.as_millis() * rest_iterations;
        println!("Graf: {} Time: {} ms ...", n, elapsed.as_millis());
        println!("Estimation Left: {} min", estimation_ms/1000/60);
    }

    println!("## Writing Benchmark ##");
    
    let mut file_csv = File::create(file_name).expect("Err. Creating report file.");
    println!("{}", data);
    file_csv.write_all(data.as_bytes()).expect("Err. Wrting report file.");

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

