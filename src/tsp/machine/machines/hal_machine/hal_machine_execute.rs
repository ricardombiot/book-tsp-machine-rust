
use crate::tsp::machine;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::machine::machines::hal_machine::Grafo;
use crate::tsp::utils::alias::{Color, Km};
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl HamiltonianMachine {
    pub fn execute(&mut self){
        // # Hamiltonian: $ O(N) $
        //# TSP: $ O(B) $
        if self.make_step() {
            println!("KM: {}/{}", self.actual_km, self.n);
            self.execute();
        }
    }

    fn make_step(&mut self) -> bool {
        if self.actual_km < self.n as Km {
            self.execute_line();
            self.actual_km += 1 as Km;
            return true;
        }else{
            return false;
        }
    }

    fn execute_line(&mut self) {
        let list_origins = self.timeline.get(&self.actual_km).unwrap().to_list_keys();
        for origin in list_origins {
            if self.is_valid_origin(origin) {
                let (is_valid, action_id) = self.timeline.execute(&mut self.db, self.actual_km, origin);
               
                if is_valid {
                    println!("Execute KM:{} Cell: {} -> OP: {} ({})", self.actual_km, origin, action_id.unwrap(), is_valid);
                    self.send_destines(&origin);
                }
            }
        }
    }

    fn is_valid_origin(&self, origin: Color) -> bool {
        //## En el ultimo caso solo calculo si tiene como destine color_origin
        if self.actual_km == (self.n-1) as Km {
            let destine = self.color_origin.clone();
            let have_arista_to_origin = self.graf.exist(origin, destine);
            return have_arista_to_origin;
        }else{
            return true;
        }
    }

}

