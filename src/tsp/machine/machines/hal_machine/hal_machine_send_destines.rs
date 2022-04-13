use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::utils::alias::{Color, Weight, Step, ActionId};
use crate::tsp::actions::action::Action;

impl HamiltonianMachine {
    pub(super) fn send_destines(&mut self, origin : &Color){
        let cell = self.timeline.get_cell(&self.actual_km, &origin).unwrap().clone();
        let action_id = cell.action_id();
        let opt_action = cell.get_action(&self.db, &action_id);
        let mut actions_to_reserve: Vec<ActionId> = Vec::new();

        match opt_action {
            Some(action) if action.valid() => {
                let parent_id = action.id();

                let list_destines: Vec<(Color, Weight)> = self.graf.get_destines(&origin);
                for (destine, weight) in list_destines {
                    if self._is_valid_destine(&action, &destine) {
                        let km_destine = self.actual_km + weight;

                        self.timeline.push_parent(&km_destine, &destine, &parent_id);
                        actions_to_reserve.push(parent_id.clone());
                    }
                }
            }
            _ => (),
        }

        self.db.reserve_destines(&actions_to_reserve);
    }

    fn _is_valid_destine(&self, action : &Action, destine : &Color) -> bool {
        if destine == &self.color_origin {
            let valid_destine_as_origin = action.max_length_graph() == self.n as Step;
            //println!("Destine origin [{}] GraphLenght {} ", valid_destine_as_origin, action.max_length_graph());
            return valid_destine_as_origin;
        }else{
            return true;
        }
        
        
    }
}

