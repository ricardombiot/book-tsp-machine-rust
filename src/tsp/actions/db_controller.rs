
use std::fmt::Debug;
use crate::tsp::utils::alias::ActionId;
use crate::tsp::actions::table_controller::TableController;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;


#[derive(Debug)]
pub struct DBController {
    table: TableController,
    pending_to_clean: Vec<ActionId>
}

impl DBController {
    pub fn new() -> Self {
        let table: TableController = TableController::new();
        let pending_to_clean: Vec<ActionId> = Vec::new();
        Self{table, pending_to_clean}
    }

    pub fn reserve(&mut self, action_id : &ActionId){
        if !self.table.have(action_id) {
            self.table.put(action_id.clone(), 0);
        }

        let value = self.table.get_mut(action_id).unwrap();
        *value = *value + 1;
    }

    pub fn use_it(&mut self, action_id : &ActionId) {
        if self.can_use_it(action_id) {
            let value = self.table.get_mut(action_id).unwrap();
            *value = *value - 1;
    
            if *value == 0 {
                self.table.delete(action_id);
                self.pending_to_clean.push(action_id.clone());
            }
        }
    }

    pub fn can_use_it(&mut self, action_id : &ActionId) -> bool {
        let value = self.table.get(action_id);

        match value {
            Some(counter) if *counter > (0 as u32) => true,
            _ => false
        }
    }

    pub fn couter_reserves(&mut self, action_id : &ActionId) -> u32 {
        let value = self.table.get(action_id);

        match value {
            Some(counter) => *counter,
            _ => 0
        }
    }

    /*
    pub fn clean_db(&mut self, db : &mut DatabaseActions){
        for action_id in self.pending_to_clean.iter() {
            db.remove(action_id);
        }

        self.clean_db_done();
    }*/

    pub fn clean_db_done(&mut self){
        self.pending_to_clean = Vec::new();
    }

    pub fn pending_to_clean_ids(&self) -> &Vec<ActionId> {
        &self.pending_to_clean
    }

}