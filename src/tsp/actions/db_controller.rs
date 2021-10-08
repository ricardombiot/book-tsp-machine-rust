
use std::fmt::Debug;
use crate::tsp::utils::alias::ActionId;
use crate::tsp::actions::table_controller::TableController;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::actions::database_actions::DatabaseActions;


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
        let value = self.table.get_mut(action_id).unwrap();
        *value = *value - 1;

        if *value == 0 {
            self.pending_to_clean.push(action_id.clone());
        }
    }

    pub fn clean_db(&mut self, db : &mut DatabaseActions){
        for node_id in self.pending_to_clean.iter() {
            db.remove(node_id);
        }

        self.pending_to_clean = Vec::new();
    }

}