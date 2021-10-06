use crate::tsp::utils::alias::{Color, Weight};
use crate::tsp::machine::components::graf::Grafo;

impl Grafo { 
    pub fn gen_complete(n : Color, peso : Weight)-> Self {
        let mut g = Grafo::new(n);

        for origen in 0..n {
            for destino in 0..n {
                if origen != destino {
                    let origin_color : Color = origen.clone() as Color;
                    let destino_color : Color = destino.clone() as Color;
                    let peso : Weight = peso.clone() as Weight;

                    g.add_bidirectional(origin_color, destino_color, peso);
                }
            }
        }

        return g;
    }

}
