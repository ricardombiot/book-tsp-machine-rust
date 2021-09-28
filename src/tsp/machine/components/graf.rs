use crate::tsp::utils::alias::{Color, Weight};

#[derive(Debug)]
pub struct Grafo {
    count_aristas : u64,
    n : Color,
    matriz : Vec<Vec<Weight>>,
    max_weight : Weight,
}

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

}

