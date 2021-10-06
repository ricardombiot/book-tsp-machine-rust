use crate::tsp::machine;
use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::machine::machines::hal_machine::Grafo;
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::utils::alias::{Color, Km, Weight, Step};
use crate::tsp::actions::action::Action;

impl HamiltonianMachine {
    pub(super) fn send_destines(&mut self, origin : &Color){
        let cell = self.timeline.get_cell(&self.actual_km, &origin).unwrap().clone();
        let opt_action = cell.get_action(&self.db);

        match opt_action {
            Some(action) if action.valid() => {
                let parent_id = action.id();

                let list_destines: Vec<(Color, Weight)> = self.graf.get_destines(&origin);
                for (destine, weight) in list_destines {
                    
                    if self._is_valid_destine(&action, &destine) {
                        let km_destine = self.actual_km + weight;

                        self.timeline.push_parent(&km_destine, &destine, &parent_id);
                    }
                }
            }
            _ => (),
        }
    }

    fn _is_valid_destine(&self, action : &Action, destine : &Color) -> bool {
        if destine == &self.color_origin {
            return action.max_length_graph() == self.n as Step;
        }else{
            return true;
        }
        
        
    }
}

