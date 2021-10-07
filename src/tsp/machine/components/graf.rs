use crate::tsp::utils::alias::{Color, Weight};

#[derive(Debug)]
pub struct Grafo {
    count_aristas : u64,
    n : Color,
    matriz : Vec<Vec<Weight>>,
    max_weight : Weight,
}

pub mod graf_generators;

impl Grafo {
    pub fn new(n : Color) -> Grafo {
        return Grafo { 
            count_aristas: 0,
            n : n,
            matriz: vec![vec![0;n] ; n],
            max_weight: 0
        }
    }

    fn is_valid_index(&self , index : Color) -> bool {
        return index < self.n;
    }

    pub fn get_n(&self) -> Color {
        return self.n
    }

    pub fn is_valid(&self, origin : Color,  destine : Color) -> bool {
        return self.is_valid_index(origin) && self.is_valid_index(destine);
    }

    fn checking_valid_index(&self, origin : Color,  destine : Color){
        if !self.is_valid(origin, destine){
            panic!("Graf: Out of index");
        }
    }

    pub fn exist(&self , origin : Color,  destine : Color) -> bool {
        self.checking_valid_index(origin, destine);

        return self.matriz[origin][destine] != 0
    }

    pub fn get_weight(&self , origin : Color,  destine : Color) -> Weight {
        self.checking_valid_index(origin, destine);

        return self.matriz[origin][destine]
    }

    pub fn get_destines(&self, origin : &Color) -> Vec<(Color, Weight)>{
        //let destines = self.matriz[origin.clone()];
        let mut list: Vec<(Color, Weight)> = Vec::new();
        for destine in 0..self.n {
            let weight : Weight = self.get_weight(origin.clone(), destine.clone());
            if weight > (0 as Weight) {
                let tuple_destine = (destine.clone(), weight);
                list.push(tuple_destine);
            }
        }
        return list;
    }

    pub fn add(&mut self, origin : Color,  destine : Color, weight : Weight){
        self.checking_valid_index(origin, destine);


        if !self.exist(origin, destine){
            self.count_aristas += 1;
        }

        if weight > self.max_weight {
            self.max_weight = weight
        }

        self.matriz[origin][destine] = weight;
    }

    pub fn add_bidirectional(&mut self, origin : Color,  destine : Color, weight : Weight){
        self.add(origin, destine, weight);
        self.add(destine, origin, weight);
    }

    pub fn add_bidirectional_vec(&mut self, origin : Color,  list_destines  : Vec<Color>, weight : Weight){
        for destine in list_destines{
            self.add_bidirectional(origin.clone(), destine, weight.clone())
        }
    }

    /*function add_bidirectional!( graf :: Grafo , origin :: Color, list_destines :: Array{Color, 1}, weight :: Weight = Weight(1))
        for destine in list_destines
            add_bidirectional!( graf, origin, destine)
        end
    end */

}

