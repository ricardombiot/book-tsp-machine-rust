use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::utils::alias::{Color, Weight};

#[test]
fn init_graf() {
    let n : Color = 10;
    let g = Grafo::new(n);
    assert_eq!(g.get_n(), n);
}

#[test]
fn add_arist(){
    let n : Color = 5;
    let mut g = Grafo::new(n);

    let weight = 10;
    g.add(4,4, weight);

    assert_eq!(g.get_weight(4,4), weight);
}

#[test]
fn test_update_weight(){
    let n : Color = 5;
    let mut g = Grafo::new(n);

    let mut fixed_weight = 1;
    for origin in 0..g.get_n()-1 {
        for destine in 0..g.get_n()-1 {   
            if origin == destine {
                let weight = g.get_weight(origin, destine);
                assert_eq!(weight, 0);

                g.add(origin, destine, fixed_weight);

                let weight = g.get_weight(origin, destine);
                assert_eq!(weight, fixed_weight);

                fixed_weight += 1;
            }
        }
    }
}


#[test]
fn test_update_weight_bidirectional(){
    let n : Color = 5;
    let mut g = Grafo::new(n);

    let mut fixed_weight : Weight = 1;
    for origin in 0..g.get_n()-1 {
        for destine in 0..g.get_n()-1 {   
            if origin == destine {
                g.add_bidirectional(origin, destine, fixed_weight);

                let weight: Weight = g.get_weight(origin, destine);
                assert_eq!(weight, fixed_weight);
                let weight: Weight = g.get_weight(destine, origin);
                assert_eq!(weight, fixed_weight);

                fixed_weight += 1;
            }
        }
    }
}


