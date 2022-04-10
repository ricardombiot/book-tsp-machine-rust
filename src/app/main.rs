//mod bench;

pub enum MainModes {
    MainDodeExample,
    MainSimpleExample,
    MainBenchQuick,
    MainBenchToN,
    MainDefault
}

use crate::app::examples;
use crate::app::bench;
use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    match args_to_mode(&args) {
        MainModes::MainDodeExample => examples::run_hal_machine_dode(),
        MainModes::MainSimpleExample => examples::run_simple_graph(),
        MainModes::MainBenchQuick => bench::run_bench_quick(&args),
        MainModes::MainBenchToN => bench::run_bench_to_max_n(&args),
        _ => show_help_info()
    }
}



fn args_to_mode(args: &Vec<String>) -> MainModes {
    println!("{:?}", args);
    if args.len() <= 1 {
        return MainModes::MainDefault;
    }else{
        return match args[1].as_str() {
            "example:dode" => MainModes::MainDodeExample,
            "example:simple" => MainModes::MainSimpleExample,
            "bench:to_n" => MainModes::MainBenchToN,
            "bench:quick" => MainModes::MainBenchQuick,
            _ => MainModes::MainDefault,
        }
    }
}

fn show_help_info(){
    println!("# Help Info #");
    println!("* bench:to_n $max_n");
    println!("  Returns a table with the times of HalMachine's execution, from 4 to $max_n.");
    println!("              Example: cargo run bench:to_n 8");
    println!("* bench:quick $n $iter $file_name");
    println!("  Executes (Instance: Complete graf - K_$n) $iter times and saves on a csv file the execution time of each one. ");
    println!("              Example: cargo run bench:quick 5 100 times_n5.part_b.csv");
    println!("* example:dode");
    println!("  Executes Dodecahedron example instance over our HalMachine");
    println!("              Example: cargo run example:dode");
    println!("* example:simple");
    println!("  Executes simple example instance over our HalMachine ");
    println!("  [Requires] dot (to print pathset) ");
    println!("              Example: cargo run example:simple");
}

